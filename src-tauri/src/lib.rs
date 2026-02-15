mod commands;
mod db;
mod error;
mod models;
mod services;

use commands::jira::JiraSettings;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize database
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data directory");

            let pool = db::init_db(app_data_dir)
                .expect("failed to initialize database");

            // Store pool in app state
            app.manage(pool);

            // Initialize Jira settings
            app.manage(Mutex::new(JiraSettings::default()));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::ping,
            commands::save_draft,
            commands::get_article,
            commands::list_articles,
            commands::delete_draft,
            commands::score_quality,
            commands::scan_sensitive_data,
            commands::list_templates,
            commands::get_template,
            commands::test_jira_connection,
            commands::save_jira_config,
            commands::fetch_jira_ticket,
            commands::search_jira_tickets,
            commands::disconnect_jira,
            commands::get_jira_connection_status,
            commands::check_ollama_status,
            commands::draft_with_llm,
            commands::test_confluence_connection,
            commands::save_confluence_config,
            commands::disconnect_confluence,
            commands::get_confluence_connection_status,
            commands::list_confluence_spaces,
            commands::publish_article,
            commands::update_published_article,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
