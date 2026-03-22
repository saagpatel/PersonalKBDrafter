use crate::db::{settings as db_settings, DbPool};
use crate::error::AppError;
use crate::models::JiraTicket;
use crate::services::{jira::JiraClient, tokens};
use tauri::State;

// Simple settings storage for URLs (PATs go in keychain)
use std::sync::Mutex;

#[derive(Default)]
pub struct JiraSettings {
    pub base_url: Option<String>,
}

async fn get_jira_base_url(
    settings: &State<'_, Mutex<JiraSettings>>,
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
        Ok(db_settings::get_setting(&conn, "jira.base_url")?)
    })
    .await
    .map_err(|e| AppError::Internal(format!("Task join error: {}", e)))??;

    let base_url =
        base_url.ok_or_else(|| AppError::Internal("Jira not configured".to_string()))?;

    let mut settings = settings
        .lock()
        .map_err(|e| AppError::Internal(format!("Failed to lock settings: {}", e)))?;
    settings.base_url = Some(base_url.clone());

    Ok(base_url)
}

#[tauri::command]
pub async fn test_jira_connection(base_url: String, pat: String) -> Result<bool, AppError> {
    let client = JiraClient::new(base_url, pat);
    client.test_connection().await
}

#[tauri::command]
pub async fn save_jira_config(
    base_url: String,
    pat: String,
    settings: State<'_, Mutex<JiraSettings>>,
    db: State<'_, DbPool>,
) -> Result<(), AppError> {
    // Store PAT in keychain
    tokens::store_token("jira", &pat)?;

    let base_url_for_db = base_url.clone();
    let pool = db.inner().clone();
    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        let conn = pool.get()?;
        Ok(db_settings::set_setting(
            &conn,
            "jira.base_url",
            &base_url_for_db,
        )?)
    })
    .await
    .map_err(|e| AppError::Internal(format!("Task join error: {}", e)))??;

    // Store base URL in app state
    let mut settings = settings.lock()
        .map_err(|e| AppError::Internal(format!("Failed to lock settings: {}", e)))?;
    settings.base_url = Some(base_url);

    Ok(())
}

#[tauri::command]
pub async fn fetch_jira_ticket(
    key: String,
    settings: State<'_, Mutex<JiraSettings>>,
    db: State<'_, DbPool>,
) -> Result<JiraTicket, AppError> {
    let base_url = get_jira_base_url(&settings, &db).await?;

    let pat = tokens::get_token("jira")?;
    let client = JiraClient::new(base_url, pat);
    client.get_ticket(&key).await
}

#[tauri::command]
pub async fn search_jira_tickets(
    query: String,
    settings: State<'_, Mutex<JiraSettings>>,
    db: State<'_, DbPool>,
) -> Result<Vec<JiraTicket>, AppError> {
    let base_url = get_jira_base_url(&settings, &db).await?;

    let pat = tokens::get_token("jira")?;
    let client = JiraClient::new(base_url, pat);
    client.search_tickets(&query).await
}

#[tauri::command]
pub async fn disconnect_jira(
    settings: State<'_, Mutex<JiraSettings>>,
    db: State<'_, DbPool>,
) -> Result<(), AppError> {
    // Delete token from keychain
    tokens::delete_token("jira")?;

    let pool = db.inner().clone();
    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        let conn = pool.get()?;
        Ok(db_settings::delete_setting(&conn, "jira.base_url")?)
    })
    .await
    .map_err(|e| AppError::Internal(format!("Task join error: {}", e)))??;

    // Clear settings
    let mut settings = settings.lock()
        .map_err(|e| AppError::Internal(format!("Failed to lock settings: {}", e)))?;
    settings.base_url = None;

    Ok(())
}

#[tauri::command]
pub async fn get_jira_connection_status(
    settings: State<'_, Mutex<JiraSettings>>,
    db: State<'_, DbPool>,
) -> Result<bool, AppError> {
    let has_url = get_jira_base_url(&settings, &db).await.is_ok();
    let has_token = tokens::get_token("jira").is_ok();

    Ok(has_url && has_token)
}
