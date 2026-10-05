use crate::models::note::{Note, NoteInput};
use crate::state::AppState;
use rusqlite::params;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn get_notes(state: State<'_, AppState>) -> Result<Vec<Note>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, title, content, is_checklist, created_at, updated_at 
             FROM notes 
             ORDER BY updated_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let notes_iter = stmt
        .query_map([], |row| {
            let is_checklist_int: i32 = row.get(3)?;
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                is_checklist: is_checklist_int != 0,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut notes = Vec::new();
    for note in notes_iter {
        notes.push(note.map_err(|e| e.to_string())?);
    }
    Ok(notes)
}

#[tauri::command]
pub fn save_note(state: State<'_, AppState>, note: NoteInput) -> Result<Note, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let is_checklist_val = if note.is_checklist { 1 } else { 0 };

    let id = match note.id {
        Some(ref existing_id) if !existing_id.trim().is_empty() => {
            // Check if note actually exists
            let count: i64 = conn
                .query_row(
                    "SELECT COUNT(1) FROM notes WHERE id = ?1",
                    params![existing_id],
                    |row| row.get(0),
                )
                .unwrap_or(0);

            if count > 0 {
                conn.execute(
                    "UPDATE notes 
                     SET title = ?1, content = ?2, is_checklist = ?3, updated_at = datetime('now', 'localtime') 
                     WHERE id = ?4",
                    params![note.title, note.content, is_checklist_val, existing_id],
                )
                .map_err(|e| e.to_string())?;
                existing_id.clone()
            } else {
                conn.execute(
                    "INSERT INTO notes (id, title, content, is_checklist, created_at, updated_at) 
                     VALUES (?1, ?2, ?3, ?4, datetime('now', 'localtime'), datetime('now', 'localtime'))",
                    params![existing_id, note.title, note.content, is_checklist_val],
                )
                .map_err(|e| e.to_string())?;
                existing_id.clone()
            }
        }
        _ => {
            let new_id = Uuid::new_v4().to_string();
            conn.execute(
                "INSERT INTO notes (id, title, content, is_checklist, created_at, updated_at) 
                 VALUES (?1, ?2, ?3, ?4, datetime('now', 'localtime'), datetime('now', 'localtime'))",
                params![new_id, note.title, note.content, is_checklist_val],
            )
            .map_err(|e| e.to_string())?;
            new_id
        }
    };

    let saved_note = conn
        .query_row(
            "SELECT id, title, content, is_checklist, created_at, updated_at 
             FROM notes 
             WHERE id = ?1",
            params![id],
            |row| {
                let is_checklist_int: i32 = row.get(3)?;
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    is_checklist: is_checklist_int != 0,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(saved_note)
}

#[tauri::command]
pub fn delete_note(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM notes WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
