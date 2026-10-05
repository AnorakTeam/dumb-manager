pub mod commands;
pub mod db;
pub mod models;
pub mod state;

use state::AppState;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let conn = db::init_db().expect("Failed to initialize SQLite database");
    let state = AppState {
        db: Mutex::new(conn),
    };

    tauri::Builder::default()
        .manage(state)
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Notes
            commands::notes::get_notes,
            commands::notes::save_note,
            commands::notes::delete_note,
            // Streaks
            commands::streaks::get_streaks,
            commands::streaks::create_streak,
            commands::streaks::reset_streak,
            commands::streaks::get_streak_history,
            commands::streaks::delete_streak,
            // Habits
            commands::habits::get_habits_with_today_progress,
            commands::habits::create_habit,
            commands::habits::increment_habit,
            commands::habits::decrement_habit,
            commands::habits::get_habit_month_logs,
            commands::habits::delete_habit,
        ])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
