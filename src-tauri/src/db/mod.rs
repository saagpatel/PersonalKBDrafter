pub mod articles;
pub mod settings;
pub mod templates;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use std::path::PathBuf;

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn init_db(app_data_dir: PathBuf) -> Result<DbPool, Box<dyn std::error::Error>> {
    // Ensure the data directory exists
    std::fs::create_dir_all(&app_data_dir)?;

    let db_path = app_data_dir.join("kb_drafter.db");
    let manager = SqliteConnectionManager::file(&db_path);
    let pool = Pool::new(manager)?;

    // Run migrations
    let conn = pool.get()?;
    run_migrations(&conn)?;

    Ok(pool)
}

fn run_migrations(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // Create migrations table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _migrations (
            id INTEGER PRIMARY KEY,
            filename TEXT NOT NULL UNIQUE,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;

    // Migration 001: Initial schema
    let migration_001 = include_str!("../../migrations/001_initial.sql");
    apply_migration(conn, "001_initial.sql", migration_001)?;

    // Migration 002: App settings
    let migration_002 = include_str!("../../migrations/002_app_settings.sql");
    apply_migration(conn, "002_app_settings.sql", migration_002)?;

    Ok(())
}

fn apply_migration(
    conn: &Connection,
    filename: &str,
    sql: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if migration already applied
    let already_applied: bool = conn
        .prepare("SELECT 1 FROM _migrations WHERE filename = ?1")?
        .exists([filename])?;

    if already_applied {
        return Ok(());
    }

    // Apply migration
    conn.execute_batch(sql)?;

    // Record migration
    conn.execute(
        "INSERT INTO _migrations (filename) VALUES (?1)",
        [filename],
    )?;

    log::info!("Applied migration: {}", filename);
    Ok(())
}
