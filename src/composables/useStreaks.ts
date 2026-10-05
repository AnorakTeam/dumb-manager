import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Streak, StreakReset, StreakSummary } from '../types/streak';

export function useStreaks() {
  const streaks = ref<StreakSummary[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchStreaks() {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<StreakSummary[]>('get_streaks');
      streaks.value = result;
    } catch (e: any) {
      error.value = e?.toString() || 'Failed to fetch streaks';
      console.error('Error fetching streaks:', e);
    } finally {
      loading.value = false;
    }
  }

  async function createStreak(title: string): Promise<Streak | null> {
    try {
      const created = await invoke<Streak>('create_streak', { title });
      await fetchStreaks();
      return created;
    } catch (e: any) {
      error.value = e?.toString() || 'Failed to create streak';
      console.error('Error creating streak:', e);
      return null;
    }
  }

  async function resetStreak(streakId: string, reason?: string): Promise<boolean> {
    try {
      await invoke('reset_streak', { streakId, reason });
      await fetchStreaks();
      return true;
    } catch (e: any) {
      error.value = e?.toString() || 'Failed to reset streak';
      console.error('Error resetting streak:', e);
      return false;
    }
  }

  async function getStreakHistory(streakId: string): Promise<StreakReset[]> {
    try {
      return await invoke<StreakReset[]>('get_streak_history', { streakId });
    } catch (e: any) {
      error.value = e?.toString() || 'Failed to fetch streak history';
      console.error('Error fetching streak history:', e);
      return [];
    }
  }

  async function deleteStreak(id: string): Promise<boolean> {
    try {
      await invoke('delete_streak', { id });
      streaks.value = streaks.value.filter((s) => s.id !== id);
      return true;
    } catch (e: any) {
      error.value = e?.toString() || 'Failed to delete streak';
      console.error('Error deleting streak:', e);
      return false;
    }
  }

  return {
    streaks,
    loading,
    error,
    fetchStreaks,
    createStreak,
    resetStreak,
    getStreakHistory,
    deleteStreak,
  };
}
