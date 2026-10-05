pub mod commands;
pub mod db;
pub mod models;
pub mod state;

use state::AppState;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

pub fn ensure_system_icon() {
    let base_dir: PathBuf = match std::env::var("XDG_DATA_HOME") {
        Ok(val) if !val.trim().is_empty() => PathBuf::from(val),
        _ => match dirs::data_dir() {
            Some(path) => path,
            None => {
                let home = std::env::var("HOME").unwrap_or_default();
                PathBuf::from(home).join(".local/share")
            }
        },
    };

    // Install 256x256 icon to hicolor
    let icon_dir = base_dir.join("icons/hicolor/256x256/apps");
    if let Ok(()) = fs::create_dir_all(&icon_dir) {
        let icon_path = icon_dir.join("dumb-manager.png");
        let _ = fs::write(&icon_path, include_bytes!("../icons/icon.png"));
    }

    // Install 128x128 icon to hicolor
    let icon_dir_128 = base_dir.join("icons/hicolor/128x128/apps");
    if let Ok(()) = fs::create_dir_all(&icon_dir_128) {
        let icon_path = icon_dir_128.join("dumb-manager.png");
        let _ = fs::write(&icon_path, include_bytes!("../icons/128x128.png"));
    }

    // Ensure desktop entry for Wayland app_id icon resolution in KWin
    let apps_dir = base_dir.join("applications");
    if let Ok(()) = fs::create_dir_all(&apps_dir) {
        let desktop_path = apps_dir.join("dumb-manager.desktop");
        if !desktop_path.exists() {
            let desktop_content = "[Desktop Entry]\nName=dumb-manager\nComment=Lightweight daily life manager\nExec=dumb-manager\nIcon=dumb-manager\nTerminal=false\nType=Application\nCategories=Utility;\nStartupWMClass=dumb-manager\n";
            let _ = fs::write(&desktop_path, desktop_content);
        }
    }
}

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

            ensure_system_icon();

            if let Some(window) = app.get_webview_window("main") {
                if let Some(icon) = app.default_window_icon() {
                    let _ = window.set_icon(icon.clone());
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Notes
            commands::notes::get_notes,
            commands::notes::save_note,
            commands::notes::delete_note,
            commands::notes::get_storage_path,
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
