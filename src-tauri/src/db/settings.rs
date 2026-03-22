use rusqlite::{params, Connection, OptionalExtension, Result as SqliteResult};

pub fn set_setting(conn: &Connection, key: &str, value: &str) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO app_settings (key, value, updated_at)
         VALUES (?1, ?2, datetime('now'))
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = datetime('now')",
        params![key, value],
    )?;

    Ok(())
}

pub fn get_setting(conn: &Connection, key: &str) -> SqliteResult<Option<String>> {
    conn.query_row(
        "SELECT value FROM app_settings WHERE key = ?1",
        [key],
        |row| row.get(0),
    )
    .optional()
}

pub fn delete_setting(conn: &Connection, key: &str) -> SqliteResult<()> {
    conn.execute("DELETE FROM app_settings WHERE key = ?1", [key])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("in-memory db");
        conn.execute_batch(include_str!("../../migrations/001_initial.sql"))
            .expect("apply initial schema");
        conn.execute_batch(include_str!("../../migrations/002_app_settings.sql"))
            .expect("apply settings schema");
        conn
    }

    #[test]
    fn test_setting_round_trip() {
        let conn = setup_conn();

        set_setting(&conn, "jira.base_url", "https://jira.example.com").expect("store setting");
        let saved = get_setting(&conn, "jira.base_url").expect("load setting");

        assert_eq!(saved.as_deref(), Some("https://jira.example.com"));
    }

    #[test]
    fn test_delete_setting() {
        let conn = setup_conn();

        set_setting(&conn, "jira.base_url", "https://jira.example.com").expect("store setting");
        delete_setting(&conn, "jira.base_url").expect("delete setting");

        let saved = get_setting(&conn, "jira.base_url").expect("load setting");
        assert!(saved.is_none());
    }
}
