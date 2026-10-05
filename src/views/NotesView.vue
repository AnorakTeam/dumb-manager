<script setup lang="ts">
import { onMounted } from 'vue';
import { Plus } from 'lucide-vue-next';
import { useNotes } from '../composables/useNotes';
import { useConfirm } from '../composables/useConfirm';
import { toast } from '../composables/useToast';
import NoteSidebar from '../components/notes/NoteSidebar.vue';
import MarkdownEditor from '../components/notes/MarkdownEditor.vue';
import type { Note, NoteInput } from '../types/note';

const {
  notes,
  activeNote,
  loading,
  fetchNotes,
  saveNote,
  deleteNote,
  setActiveNote,
} = useNotes();

const { confirm } = useConfirm();

onMounted(() => {
  fetchNotes();
});

async function handleCreate() {
  const newNote = await saveNote({
    title: 'Nueva Nota',
    content: '',
    is_checklist: false,
  });
  if (newNote) {
    setActiveNote(newNote);
    toast.success('Nueva nota creada');
  }
}

function handleSelect(note: Note) {
  setActiveNote(note);
}

async function handleSave(input: NoteInput) {
  await saveNote(input);
}

async function handleDelete(id: string) {
  const note = notes.value.find((n) => n.id === id);
  const noteTitle = note?.title?.trim() || 'esta nota';
  const ok = await confirm({
    title: '¿Eliminar nota permanentemente?',
    message: `¿Estás seguro de eliminar "${noteTitle}"? Esta acción borrará la nota de la base de datos local y no se puede deshacer.`,
    confirmText: 'Eliminar',
    cancelText: 'Cancelar',
    type: 'danger',
  });
  if (ok) {
    const success = await deleteNote(id);
    if (success) {
      toast.success('Nota eliminada');
    }
  }
}
</script>

<template>
  <div class="h-full w-full flex overflow-hidden">
    <!-- Sidebar -->
    <NoteSidebar
      :notes="notes"
      :active-note-id="activeNote?.id"
      :loading="loading"
      @select="handleSelect"
      @create="handleCreate"
      @delete="handleDelete"
    />

    <!-- Editor Area or Empty State -->
    <main class="flex-1 h-full min-w-0 flex flex-col overflow-hidden">
      <MarkdownEditor
        v-if="activeNote"
        :key="activeNote.id"
        :note="activeNote"
        @save="handleSave"
        @delete="handleDelete"
      />
      <div
        v-else
        class="flex-1 flex flex-col items-center justify-center p-8 text-center bg-slate-950 text-slate-400 select-none"
      >
        <div class="relative mb-4">
          <img
            src="/logo-avatar.png"
            alt="dumb-manager"
            class="w-60 h-60 rounded-2xl object-cover border border-slate-800 shadow-xl shadow-sky-500/10"
          />
        </div>
        <h3 class="text-base font-semibold text-slate-200">No hay ninguna nota seleccionada</h3>
        <p class="text-xs text-slate-400 mt-1 max-w-sm">
          Crea una nueva nota en Markdown o selecciona una existente de la barra lateral para empezar a escribir.
        </p>
        <button
          @click="handleCreate"
          class="mt-4 flex items-center gap-2 px-4 py-2 bg-sky-500 hover:bg-sky-400 text-slate-950 font-semibold text-xs rounded-lg shadow-sm transition-colors cursor-pointer"
        >
          <Plus class="w-4 h-4" />
          <span>Crear Nota</span>
        </button>
      </div>
    </main>
  </div>
</template>
