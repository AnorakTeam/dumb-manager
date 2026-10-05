use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_checklist: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct NoteInput {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub is_checklist: bool,
}
