use crate::models::streak::{Streak, StreakReset, StreakSummary};
use crate::state::AppState;
use rusqlite::params;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn get_streaks(state: State<'_, AppState>) -> Result<Vec<StreakSummary>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    // Query each streak with latest reset and reset count
    let mut stmt = conn
        .prepare(
            "SELECT 
                s.id, 
                s.title, 
                s.created_at,
                (SELECT r.reset_at FROM streak_resets r WHERE r.streak_id = s.id ORDER BY r.reset_at DESC LIMIT 1) as last_reset_at,
                (SELECT r.reason FROM streak_resets r WHERE r.streak_id = s.id ORDER BY r.reset_at DESC LIMIT 1) as last_reason,
                (SELECT COUNT(1) FROM streak_resets r WHERE r.streak_id = s.id) as total_resets
             FROM streaks s
             ORDER BY s.created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let streaks_iter = stmt
        .query_map([], |row| {
            let id: String = row.get(0)?;
            let title: String = row.get(1)?;
            let created_at: String = row.get(2)?;
            let last_reset_at: Option<String> = row.get(3)?;
            let last_reason: Option<String> = row.get(4)?;
            let total_resets: i64 = row.get(5)?;

            let effective_since = match &last_reset_at {
                Some(reset_time) => reset_time.clone(),
                None => created_at.clone(),
            };

            Ok(StreakSummary {
                id,
                title,
                created_at,
                last_reset_at,
                last_reason,
                effective_since,
                total_resets,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut list = Vec::new();
    for streak in streaks_iter {
        list.push(streak.map_err(|e| e.to_string())?);
    }
    Ok(list)
}

#[tauri::command]
pub fn create_streak(state: State<'_, AppState>, title: String) -> Result<Streak, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let new_id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO streaks (id, title, created_at) 
         VALUES (?1, ?2, datetime('now', 'localtime'))",
        params![new_id, title.trim()],
    )
    .map_err(|e| e.to_string())?;

    let streak = conn
        .query_row(
            "SELECT id, title, created_at FROM streaks WHERE id = ?1",
            params![new_id],
            |row| {
                Ok(Streak {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    created_at: row.get(2)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(streak)
}

#[tauri::command]
pub fn reset_streak(
    state: State<'_, AppState>,
    streak_id: String,
    reason: Option<String>,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let new_id = Uuid::new_v4().to_string();
    let reason_val = reason.unwrap_or_default();

    conn.execute(
        "INSERT INTO streak_resets (id, streak_id, reset_at, reason) 
         VALUES (?1, ?2, datetime('now', 'localtime'), ?3)",
        params![new_id, streak_id, reason_val.trim()],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_streak_history(
    state: State<'_, AppState>,
    streak_id: String,
) -> Result<Vec<StreakReset>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, streak_id, reset_at, reason 
             FROM streak_resets 
             WHERE streak_id = ?1 
             ORDER BY reset_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let resets_iter = stmt
        .query_map(params![streak_id], |row| {
            Ok(StreakReset {
                id: row.get(0)?,
                streak_id: row.get(1)?,
                reset_at: row.get(2)?,
                reason: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut resets = Vec::new();
    for r in resets_iter {
        resets.push(r.map_err(|e| e.to_string())?);
    }
    Ok(resets)
}

#[tauri::command]
pub fn delete_streak(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM streaks WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
