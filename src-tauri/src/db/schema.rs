pub const SCHEMA_SQL: &str = r#"
PRAGMA foreign_keys = ON;

-- 1. Notas
CREATE TABLE IF NOT EXISTS notes (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL DEFAULT '',
    is_checklist INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

-- 2. Rachas ("Days Since")
CREATE TABLE IF NOT EXISTS streaks (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

CREATE TABLE IF NOT EXISTS streak_resets (
    id TEXT PRIMARY KEY NOT NULL,
    streak_id TEXT NOT NULL,
    reset_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    reason TEXT DEFAULT '',
    FOREIGN KEY (streak_id) REFERENCES streaks(id) ON DELETE CASCADE
);

-- 3. Hábitos y Progreso Diario
CREATE TABLE IF NOT EXISTS habits (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    daily_target INTEGER NOT NULL DEFAULT 1,
    color TEXT DEFAULT '#3b82f6',
    created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

CREATE TABLE IF NOT EXISTS habit_logs (
    id TEXT PRIMARY KEY NOT NULL,
    habit_id TEXT NOT NULL,
    date TEXT NOT NULL,
    current_count INTEGER NOT NULL DEFAULT 0,
    UNIQUE(habit_id, date),
    FOREIGN KEY (habit_id) REFERENCES habits(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_habit_logs_date ON habit_logs(habit_id, date);
CREATE INDEX IF NOT EXISTS idx_streak_resets_streak ON streak_resets(streak_id);
"#;
