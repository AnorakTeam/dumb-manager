export interface Habit {
  id: string;
  title: string;
  daily_target: number;
  color: string;
  created_at: string;
}

export interface HabitLog {
  id: string;
  habit_id: string;
  date: string;
  current_count: number;
}

export interface HabitProgress {
  id: string;
  title: string;
  daily_target: number;
  color: string;
  created_at: string;
  today_count: number;
  is_completed: boolean;
}
