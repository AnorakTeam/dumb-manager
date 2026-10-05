use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Streak {
    pub id: String,
    pub title: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StreakReset {
    pub id: String,
    pub streak_id: String,
    pub reset_at: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StreakSummary {
    pub id: String,
    pub title: String,
    pub created_at: String,
    pub last_reset_at: Option<String>,
    pub last_reason: Option<String>,
    pub effective_since: String,
    pub total_resets: i64,
}
