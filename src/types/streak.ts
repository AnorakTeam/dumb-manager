export interface Streak {
  id: string;
  title: string;
  created_at: string;
}

export interface StreakReset {
  id: string;
  streak_id: string;
  reset_at: string;
  reason: string;
}

export interface StreakSummary {
  id: string;
  title: string;
  created_at: string;
  last_reset_at: string | null;
  last_reason: string | null;
  effective_since: string;
  total_resets: number;
}
