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
) -> Result<(), AppError> {
    // Store PAT in keychain
    tokens::store_token("jira", &pat)?;

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
) -> Result<JiraTicket, AppError> {
    let base_url = {
        let settings = settings.lock()
            .map_err(|e| AppError::Internal(format!("Failed to lock settings: {}", e)))?;
        settings
            .base_url
            .clone()
            .ok_or_else(|| AppError::Internal("Jira not configured".to_string()))?
    };

    let pat = tokens::get_token("jira")?;
    let client = JiraClient::new(base_url, pat);
    client.get_ticket(&key).await
}

#[tauri::command]
pub async fn search_jira_tickets(
    query: String,
    settings: State<'_, Mutex<JiraSettings>>,
) -> Result<Vec<JiraTicket>, AppError> {
    let base_url = {
        let settings = settings.lock()
            .map_err(|e| AppError::Internal(format!("Failed to lock settings: {}", e)))?;
        settings
            .base_url
            .clone()
            .ok_or_else(|| AppError::Internal("Jira not configured".to_string()))?
    };

    let pat = tokens::get_token("jira")?;
    let client = JiraClient::new(base_url, pat);
    client.search_tickets(&query).await
}

#[tauri::command]
pub async fn disconnect_jira(
    settings: State<'_, Mutex<JiraSettings>>,
) -> Result<(), AppError> {
    // Delete token from keychain
    tokens::delete_token("jira")?;

    // Clear settings
    let mut settings = settings.lock()
        .map_err(|e| AppError::Internal(format!("Failed to lock settings: {}", e)))?;
    settings.base_url = None;

    Ok(())
}

#[tauri::command]
pub async fn get_jira_connection_status(
    settings: State<'_, Mutex<JiraSettings>>,
) -> Result<bool, AppError> {
    let settings = settings.lock()
        .map_err(|e| AppError::Internal(format!("Failed to lock settings: {}", e)))?;
    let has_url = settings.base_url.is_some();
    let has_token = tokens::get_token("jira").is_ok();

    Ok(has_url && has_token)
}
