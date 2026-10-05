use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Habit {
    pub id: String,
    pub title: String,
    pub daily_target: i32,
    pub color: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HabitLog {
    pub id: String,
    pub habit_id: String,
    pub date: String,
    pub current_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HabitProgress {
    pub id: String,
    pub title: String,
    pub daily_target: i32,
    pub color: String,
    pub created_at: String,
    pub today_count: i32,
    pub is_completed: bool,
}
