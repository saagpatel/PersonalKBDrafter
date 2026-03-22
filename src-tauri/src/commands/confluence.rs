use crate::db::{settings as db_settings, DbPool};
use crate::error::AppError;
use crate::models::confluence::{ConfluenceSpace, PublishResult};
use crate::services::{confluence::ConfluenceClient, markdown_to_confluence, tokens};
use std::sync::Mutex;
use tauri::State;

#[derive(Default)]
pub struct ConfluenceSettings {
    pub base_url: Option<String>,
}

async fn get_confluence_base_url(
    settings: &State<'_, Mutex<ConfluenceSettings>>,
    db: &State<'_, DbPool>,
) -> Result<String, AppError> {
    {
        let settings = settings
            .lock()
            .map_err(|e| AppError::Internal(format!("Failed to lock settings: {}", e)))?;
        if let Some(base_url) = settings.base_url.clone() {
            return Ok(base_url);
        }
    }

    let pool = db.inner().clone();
    let base_url = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let conn = pool.get()?;
        Ok(db_settings::get_setting(&conn, "confluence.base_url")?)
    })
    .await
    .map_err(|e| AppError::Internal(format!("Task join error: {}", e)))??;

    let base_url =
        base_url.ok_or_else(|| AppError::Internal("Confluence not configured".to_string()))?;

    let mut settings = settings
        .lock()
        .map_err(|e| AppError::Internal(format!("Failed to lock settings: {}", e)))?;
    settings.base_url = Some(base_url.clone());

    Ok(base_url)
}

async fn resolve_confluence_base_url(
    requested_base_url: String,
    settings: &State<'_, Mutex<ConfluenceSettings>>,
    db: &State<'_, DbPool>,
) -> Result<String, AppError> {
    if !requested_base_url.trim().is_empty() {
        return Ok(requested_base_url);
    }

    get_confluence_base_url(settings, db).await
}

/// Test Confluence connection
#[tauri::command]
pub async fn test_confluence_connection(
    base_url: String,
    pat: String,
) -> Result<bool, AppError> {
    let client = ConfluenceClient::new(base_url, pat);
    client.test_connection().await
}

/// Save Confluence configuration
#[tauri::command]
pub async fn save_confluence_config(
    base_url: String,
    pat: String,
    settings: State<'_, Mutex<ConfluenceSettings>>,
    db: State<'_, DbPool>,
) -> Result<(), AppError> {
    // Store PAT in keychain
    tokens::store_token("confluence", &pat)?;

    let base_url_for_db = base_url.clone();
    let pool = db.inner().clone();
    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        let conn = pool.get()?;
        Ok(db_settings::set_setting(
            &conn,
            "confluence.base_url",
            &base_url_for_db,
        )?)
    })
    .await
    .map_err(|e| AppError::Internal(format!("Task join error: {}", e)))??;

    let mut settings = settings
        .lock()
        .map_err(|e| AppError::Internal(format!("Failed to lock settings: {}", e)))?;
    settings.base_url = Some(base_url);

    Ok(())
}

/// Disconnect from Confluence
#[tauri::command]
pub async fn disconnect_confluence(
    settings: State<'_, Mutex<ConfluenceSettings>>,
    db: State<'_, DbPool>,
) -> Result<(), AppError> {
    tokens::delete_token("confluence")?;

    let pool = db.inner().clone();
    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        let conn = pool.get()?;
        Ok(db_settings::delete_setting(&conn, "confluence.base_url")?)
    })
    .await
    .map_err(|e| AppError::Internal(format!("Task join error: {}", e)))??;

    let mut settings = settings
        .lock()
        .map_err(|e| AppError::Internal(format!("Failed to lock settings: {}", e)))?;
    settings.base_url = None;

    Ok(())
}

/// Get Confluence connection status
#[tauri::command]
pub async fn get_confluence_connection_status(
    settings: State<'_, Mutex<ConfluenceSettings>>,
    db: State<'_, DbPool>,
) -> Result<bool, AppError> {
    let has_url = get_confluence_base_url(&settings, &db).await.is_ok();
    let has_token = tokens::get_token("confluence").is_ok();

    Ok(has_url && has_token)
}

/// List available Confluence spaces
#[tauri::command]
pub async fn list_confluence_spaces(
    confluence_url: String,
    settings: State<'_, Mutex<ConfluenceSettings>>,
    db: State<'_, DbPool>,
) -> Result<Vec<ConfluenceSpace>, AppError> {
    let confluence_url = resolve_confluence_base_url(confluence_url, &settings, &db).await?;
    let pat = tokens::get_token("confluence")?;
    let client = ConfluenceClient::new(confluence_url, pat);
    client.list_spaces().await
}

/// Publish an article to Confluence
#[tauri::command]
pub async fn publish_article(
    article_id: i64,
    space_key: String,
    confluence_url: String,
    settings: State<'_, Mutex<ConfluenceSettings>>,
    db: State<'_, DbPool>,
) -> Result<PublishResult, AppError> {
    let confluence_url = resolve_confluence_base_url(confluence_url, &settings, &db).await?;
    let pat = tokens::get_token("confluence")?;

    // Get article from database
    let pool = db.inner().clone();
    let article = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let conn = pool.get()?;
        Ok(crate::db::articles::get_article(&conn, article_id)?)
    })
    .await
    .map_err(|e| AppError::Internal(format!("Task join error: {}", e)))??;

    // Convert markdown to Confluence XHTML
    let conversion_result = markdown_to_confluence::convert(&article.content_markdown)?;

    // Create page in Confluence
    let client = ConfluenceClient::new(confluence_url, pat);
    let publish_result = client
        .create_page(&space_key, &article.title, &conversion_result.xhtml, &article.tags)
        .await?;

    // Update article in database with publish info
    let pool2 = db.inner().clone();
    let page_id = publish_result.page_id.clone();
    let page_url = publish_result.url.clone();
    let space_key_clone = space_key.clone();

    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        let conn = pool2.get()?;
        conn.execute(
            "UPDATE kb_articles SET status = 'published', confluence_page_id = ?1, confluence_url = ?2, confluence_space_key = ?3 WHERE id = ?4",
            (page_id, page_url, space_key_clone, article_id),
        )?;
        Ok(())
    })
    .await
    .map_err(|e| AppError::Internal(format!("Task join error: {}", e)))??;

    Ok(publish_result)
}

/// Update an already-published article in Confluence
#[tauri::command]
pub async fn update_published_article(
    article_id: i64,
    confluence_url: String,
    settings: State<'_, Mutex<ConfluenceSettings>>,
    db: State<'_, DbPool>,
) -> Result<PublishResult, AppError> {
    let confluence_url = resolve_confluence_base_url(confluence_url, &settings, &db).await?;
    let pat = tokens::get_token("confluence")?;

    // Get article from database
    let pool = db.inner().clone();
    let article = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let conn = pool.get()?;
        Ok(crate::db::articles::get_article(&conn, article_id)?)
    })
    .await
    .map_err(|e| AppError::Internal(format!("Task join error: {}", e)))??;

    // Check if article has been published
    let page_id = article.confluence_page_id.ok_or_else(|| {
        AppError::Internal("Article has not been published yet".to_string())
    })?;

    // Convert markdown to Confluence XHTML
    let conversion_result = markdown_to_confluence::convert(&article.content_markdown)?;

    // Fetch current page version, then update
    let client = ConfluenceClient::new(confluence_url, pat);
    let current_version = client.get_page_version(&page_id).await?;
    let publish_result = client
        .update_page(&page_id, &article.title, &conversion_result.xhtml, current_version)
        .await?;

    // Update article URL in database
    let pool2 = db.inner().clone();
    let page_url = publish_result.url.clone();

    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        let conn = pool2.get()?;
        conn.execute(
            "UPDATE kb_articles SET confluence_url = ?1 WHERE id = ?2",
            (page_url, article_id),
        )?;
        Ok(())
    })
    .await
    .map_err(|e| AppError::Internal(format!("Task join error: {}", e)))??;

    Ok(publish_result)
}
