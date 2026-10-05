pub mod schema;

use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;

pub fn init_db() -> Result<Connection, String> {
    let base_dir: PathBuf = match std::env::var("XDG_DATA_HOME") {
        Ok(val) if !val.trim().is_empty() => PathBuf::from(val),
        _ => match dirs::data_dir() {
            Some(path) => path,
            None => {
                let home = std::env::var("HOME").map_err(|e| format!("Failed to read HOME: {}", e))?;
                PathBuf::from(home).join(".local/share")
            }
        },
    };

    let app_dir = base_dir.join("dumb-manager");
    fs::create_dir_all(&app_dir)
        .map_err(|e| format!("Failed to create data directory {:?}: {}", app_dir, e))?;

    let db_path = app_dir.join("dumb.db");
    log::info!("Connecting to SQLite database at {:?}", db_path);

    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open SQLite database at {:?}: {}", db_path, e))?;

    conn.execute_batch(schema::SCHEMA_SQL)
        .map_err(|e| format!("Failed to initialize database schema: {}", e))?;

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_initialization() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(schema::SCHEMA_SQL).unwrap();

        // Check tables exist
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap();
        let tables: Vec<String> = stmt
            .query_map([], |row| row.get(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        assert!(tables.contains(&"notes".to_string()));
        assert!(tables.contains(&"streaks".to_string()));
        assert!(tables.contains(&"streak_resets".to_string()));
        assert!(tables.contains(&"habits".to_string()));
        assert!(tables.contains(&"habit_logs".to_string()));
    }
}
