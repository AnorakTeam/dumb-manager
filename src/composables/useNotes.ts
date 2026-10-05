import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Note, NoteInput } from '../types/note';

export function useNotes() {
  const notes = ref<Note[]>([]);
  const activeNote = ref<Note | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchNotes() {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<Note[]>('get_notes');
      notes.value = result;
      if (activeNote.value) {
        const found = result.find((n) => n.id === activeNote.value?.id);
        if (found) {
          activeNote.value = found;
        } else if (result.length > 0) {
          activeNote.value = result[0];
        } else {
          activeNote.value = null;
        }
      } else if (result.length > 0) {
        activeNote.value = result[0];
      }
    } catch (e: any) {
      error.value = e?.toString() || 'Failed to fetch notes';
      console.error('Error fetching notes:', e);
    } finally {
      loading.value = false;
    }
  }

  async function saveNote(input: NoteInput): Promise<Note | null> {
    try {
      const saved = await invoke<Note>('save_note', { note: input });
      const idx = notes.value.findIndex((n) => n.id === saved.id);
      if (idx >= 0) {
        notes.value[idx] = saved;
      } else {
        notes.value.unshift(saved);
      }
      if (activeNote.value?.id === saved.id || !activeNote.value) {
        activeNote.value = saved;
      }
      return saved;
    } catch (e: any) {
      error.value = e?.toString() || 'Failed to save note';
      console.error('Error saving note:', e);
      return null;
    }
  }

  async function deleteNote(id: string): Promise<boolean> {
    try {
      await invoke('delete_note', { id });
      notes.value = notes.value.filter((n) => n.id !== id);
      if (activeNote.value?.id === id) {
        activeNote.value = notes.value.length > 0 ? notes.value[0] : null;
      }
      return true;
    } catch (e: any) {
      error.value = e?.toString() || 'Failed to delete note';
      console.error('Error deleting note:', e);
      return false;
    }
  }

  function setActiveNote(note: Note | null) {
    activeNote.value = note;
  }

  return {
    notes,
    activeNote,
    loading,
    error,
    fetchNotes,
    saveNote,
    deleteNote,
    setActiveNote,
  };
}
