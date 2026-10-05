use crate::models::habit::{Habit, HabitLog, HabitProgress};
use crate::state::AppState;
use chrono::Local;
use rusqlite::params;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn get_habits_with_today_progress(
    state: State<'_, AppState>,
) -> Result<Vec<HabitProgress>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let today = Local::now().format("%Y-%m-%d").to_string();

    let mut stmt = conn
        .prepare(
            "SELECT 
                h.id, 
                h.title, 
                h.daily_target, 
                COALESCE(h.color, '#3b82f6'), 
                h.created_at, 
                COALESCE(l.current_count, 0)
             FROM habits h
             LEFT JOIN habit_logs l ON l.habit_id = h.id AND l.date = ?1
             ORDER BY h.created_at ASC",
        )
        .map_err(|e| e.to_string())?;

    let habits_iter = stmt
        .query_map(params![today], |row| {
            let id: String = row.get(0)?;
            let title: String = row.get(1)?;
            let daily_target: i32 = row.get(2)?;
            let color: String = row.get(3)?;
            let created_at: String = row.get(4)?;
            let today_count: i32 = row.get(5)?;
            let is_completed = today_count >= daily_target;

            Ok(HabitProgress {
                id,
                title,
                daily_target,
                color,
                created_at,
                today_count,
                is_completed,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut list = Vec::new();
    for h in habits_iter {
        list.push(h.map_err(|e| e.to_string())?);
    }
    Ok(list)
}

#[tauri::command]
pub fn create_habit(
    state: State<'_, AppState>,
    title: String,
    daily_target: i32,
    color: Option<String>,
) -> Result<Habit, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let new_id = Uuid::new_v4().to_string();
    let color_val = color.unwrap_or_else(|| "#3b82f6".to_string());
    let target = if daily_target < 1 { 1 } else { daily_target };

    conn.execute(
        "INSERT INTO habits (id, title, daily_target, color, created_at) 
         VALUES (?1, ?2, ?3, ?4, datetime('now', 'localtime'))",
        params![new_id, title.trim(), target, color_val.trim()],
    )
    .map_err(|e| e.to_string())?;

    let habit = conn
        .query_row(
            "SELECT id, title, daily_target, color, created_at FROM habits WHERE id = ?1",
            params![new_id],
            |row| {
                Ok(Habit {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    daily_target: row.get(2)?,
                    color: row.get(3)?,
                    created_at: row.get(4)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(habit)
}

#[tauri::command]
pub fn increment_habit(
    state: State<'_, AppState>,
    habit_id: String,
    date: String,
) -> Result<HabitLog, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let log_id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO habit_logs (id, habit_id, date, current_count) 
         VALUES (?1, ?2, ?3, 1) 
         ON CONFLICT(habit_id, date) DO UPDATE SET current_count = current_count + 1",
        params![log_id, habit_id, date],
    )
    .map_err(|e| e.to_string())?;

    let log = conn
        .query_row(
            "SELECT id, habit_id, date, current_count FROM habit_logs WHERE habit_id = ?1 AND date = ?2",
            params![habit_id, date],
            |row| {
                Ok(HabitLog {
                    id: row.get(0)?,
                    habit_id: row.get(1)?,
                    date: row.get(2)?,
                    current_count: row.get(3)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(log)
}

#[tauri::command]
pub fn decrement_habit(
    state: State<'_, AppState>,
    habit_id: String,
    date: String,
) -> Result<HabitLog, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let log_id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO habit_logs (id, habit_id, date, current_count) 
         VALUES (?1, ?2, ?3, 0) 
         ON CONFLICT(habit_id, date) DO UPDATE SET current_count = MAX(0, current_count - 1)",
        params![log_id, habit_id, date],
    )
    .map_err(|e| e.to_string())?;

    let log = conn
        .query_row(
            "SELECT id, habit_id, date, current_count FROM habit_logs WHERE habit_id = ?1 AND date = ?2",
            params![habit_id, date],
            |row| {
                Ok(HabitLog {
                    id: row.get(0)?,
                    habit_id: row.get(1)?,
                    date: row.get(2)?,
                    current_count: row.get(3)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(log)
}

#[tauri::command]
pub fn get_habit_month_logs(
    state: State<'_, AppState>,
    habit_id: String,
    year: i32,
    month: i32,
) -> Result<Vec<HabitLog>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let pattern = format!("{:04}-{:02}-%", year, month);

    let mut stmt = conn
        .prepare(
            "SELECT id, habit_id, date, current_count 
             FROM habit_logs 
             WHERE habit_id = ?1 AND date LIKE ?2 
             ORDER BY date ASC",
        )
        .map_err(|e| e.to_string())?;

    let logs_iter = stmt
        .query_map(params![habit_id, pattern], |row| {
            Ok(HabitLog {
                id: row.get(0)?,
                habit_id: row.get(1)?,
                date: row.get(2)?,
                current_count: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut logs = Vec::new();
    for l in logs_iter {
        logs.push(l.map_err(|e| e.to_string())?);
    }
    Ok(logs)
}

#[tauri::command]
pub fn delete_habit(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM habits WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
