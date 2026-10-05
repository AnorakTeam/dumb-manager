pub mod habits;
pub mod notes;
pub mod streaks;

#[cfg(test)]
mod tests {
    use crate::db::schema::SCHEMA_SQL;
    use crate::state::AppState;
    use rusqlite::Connection;
    use std::sync::Mutex;

    // Helper to build test state
    fn setup_test_state() -> AppState {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(SCHEMA_SQL).unwrap();
        AppState {
            db: Mutex::new(conn),
        }
    }

    #[test]
    fn test_note_lifecycle() {
        let state = setup_test_state();

        // 1. Save new note
        let conn = state.db.lock().unwrap();
        let new_id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO notes (id, title, content, is_checklist) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![new_id, "Test Note", "- [ ] Task 1\n- [x] Task 2", 1],
        )
        .unwrap();

        let count: i64 = conn
            .query_row("SELECT COUNT(1) FROM notes", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_habit_increment_decrement_floor() {
        let state = setup_test_state();
        let conn = state.db.lock().unwrap();

        // Create habit
        let habit_id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO habits (id, title, daily_target, color) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![habit_id, "Agua", 3, "#38bdf8"],
        )
        .unwrap();

        let date = "2026-10-04";

        // Increment 1
        let log_id1 = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO habit_logs (id, habit_id, date, current_count) VALUES (?1, ?2, ?3, 1)
             ON CONFLICT(habit_id, date) DO UPDATE SET current_count = current_count + 1",
            rusqlite::params![log_id1, habit_id, date],
        )
        .unwrap();

        // Increment 2
        let log_id2 = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO habit_logs (id, habit_id, date, current_count) VALUES (?1, ?2, ?3, 1)
             ON CONFLICT(habit_id, date) DO UPDATE SET current_count = current_count + 1",
            rusqlite::params![log_id2, habit_id, date],
        )
        .unwrap();

        let count: i32 = conn
            .query_row(
                "SELECT current_count FROM habit_logs WHERE habit_id = ?1 AND date = ?2",
                rusqlite::params![habit_id, date],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, 2);

        // Decrement with floor at 0
        let log_id3 = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO habit_logs (id, habit_id, date, current_count) VALUES (?1, ?2, ?3, 0)
             ON CONFLICT(habit_id, date) DO UPDATE SET current_count = MAX(0, current_count - 1)",
            rusqlite::params![log_id3, habit_id, date],
        )
        .unwrap();

        let count: i32 = conn
            .query_row(
                "SELECT current_count FROM habit_logs WHERE habit_id = ?1 AND date = ?2",
                rusqlite::params![habit_id, date],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);

        // Decrement twice more to test floor 0
        for _ in 0..2 {
            conn.execute(
                "INSERT INTO habit_logs (id, habit_id, date, current_count) VALUES (?1, ?2, ?3, 0)
                 ON CONFLICT(habit_id, date) DO UPDATE SET current_count = MAX(0, current_count - 1)",
                rusqlite::params![uuid::Uuid::new_v4().to_string(), habit_id, date],
            )
            .unwrap();
        }

        let count: i32 = conn
            .query_row(
                "SELECT current_count FROM habit_logs WHERE habit_id = ?1 AND date = ?2",
                rusqlite::params![habit_id, date],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, 0);
    }
}
