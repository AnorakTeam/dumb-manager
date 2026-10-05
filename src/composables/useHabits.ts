import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Habit, HabitLog, HabitProgress } from '../types/habit';

export function useHabits() {
  const habits = ref<HabitProgress[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchHabits() {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<HabitProgress[]>('get_habits_with_today_progress');
      habits.value = result;
    } catch (e: any) {
      error.value = e?.toString() || 'Failed to fetch habits';
      console.error('Error fetching habits:', e);
    } finally {
      loading.value = false;
    }
  }

  async function createHabit(title: string, dailyTarget: number, color?: string): Promise<Habit | null> {
    try {
      const created = await invoke<Habit>('create_habit', {
        title,
        dailyTarget,
        color: color || '#3b82f6',
      });
      await fetchHabits();
      return created;
    } catch (e: any) {
      error.value = e?.toString() || 'Failed to create habit';
      console.error('Error creating habit:', e);
      return null;
    }
  }

  async function incrementHabit(habitId: string, date: string): Promise<HabitLog | null> {
    try {
      const log = await invoke<HabitLog>('increment_habit', { habitId, date });
      // Update local habit progress optimistically / synchronously
      const found = habits.value.find((h) => h.id === habitId);
      if (found) {
        found.today_count = log.current_count;
        found.is_completed = found.today_count >= found.daily_target;
      }
      return log;
    } catch (e: any) {
      error.value = e?.toString() || 'Failed to increment habit';
      console.error('Error incrementing habit:', e);
      return null;
    }
  }

  async function decrementHabit(habitId: string, date: string): Promise<HabitLog | null> {
    try {
      const log = await invoke<HabitLog>('decrement_habit', { habitId, date });
      const found = habits.value.find((h) => h.id === habitId);
      if (found) {
        found.today_count = log.current_count;
        found.is_completed = found.today_count >= found.daily_target;
      }
      return log;
    } catch (e: any) {
      error.value = e?.toString() || 'Failed to decrement habit';
      console.error('Error decrementing habit:', e);
      return null;
    }
  }

  async function getMonthLogs(habitId: string, year: number, month: number): Promise<HabitLog[]> {
    try {
      return await invoke<HabitLog[]>('get_habit_month_logs', { habitId, year, month });
    } catch (e: any) {
      error.value = e?.toString() || 'Failed to get month logs';
      console.error('Error fetching month logs:', e);
      return [];
    }
  }

  async function deleteHabit(id: string): Promise<boolean> {
    try {
      await invoke('delete_habit', { id });
      habits.value = habits.value.filter((h) => h.id !== id);
      return true;
    } catch (e: any) {
      error.value = e?.toString() || 'Failed to delete habit';
      console.error('Error deleting habit:', e);
      return false;
    }
  }

  return {
    habits,
    loading,
    error,
    fetchHabits,
    createHabit,
    incrementHabit,
    decrementHabit,
    getMonthLogs,
    deleteHabit,
  };
}
