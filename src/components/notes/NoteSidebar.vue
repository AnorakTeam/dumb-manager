<script setup lang="ts">
import { ref, computed } from 'vue';
import { Plus, Search, Trash2, FileText, X } from 'lucide-vue-next';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import 'dayjs/locale/es';
import type { Note } from '../../types/note';

dayjs.extend(relativeTime);
dayjs.locale('es');

const props = defineProps<{
  notes: Note[];
  activeNoteId?: string;
  loading: boolean;
}>();

const emit = defineEmits<{
  (e: 'select', note: Note): void;
  (e: 'create'): void;
  (e: 'delete', id: string): void;
}>();

const searchQuery = ref('');

const filteredNotes = computed(() => {
  if (!searchQuery.value.trim()) return props.notes;
  const q = searchQuery.value.toLowerCase();
  return props.notes.filter(
    (note) =>
      note.title.toLowerCase().includes(q) ||
      note.content.toLowerCase().includes(q)
  );
});

function formatTime(dateStr: string) {
  try {
    return dayjs(dateStr).fromNow();
  } catch {
    return dateStr;
  }
}

function getSnippet(content: string) {
  if (!content) return 'Nota vacía...';
  const firstLine = content.split('\n').find((line) => line.trim().length > 0);
  return firstLine ? firstLine.replace(/^[#\-*>\s\[\]x]+/, '').trim() || 'Nota sin texto' : 'Nota vacía...';
}

function handleDelete(e: Event, id: string) {
  e.stopPropagation();
  if (confirm('¿Eliminar esta nota permanentemente?')) {
    emit('delete', id);
  }
}
</script>

<template>
  <div class="w-80 h-full bg-slate-900/90 border-r border-slate-800 flex flex-col select-none shrink-0">
    <!-- Top actions & Search -->
    <div class="p-3 border-b border-slate-800/80 space-y-2.5">
      <div class="flex items-center justify-between">
        <span class="text-xs font-semibold text-slate-300 uppercase tracking-wider">
          Notas ({{ filteredNotes.length }})
        </span>
        <button
          @click="emit('create')"
          class="flex items-center gap-1.5 px-2.5 py-1.5 bg-sky-500 hover:bg-sky-400 text-slate-950 font-medium text-xs rounded-lg shadow-sm transition-colors cursor-pointer"
        >
          <Plus class="w-3.5 h-3.5" />
          <span>Nueva Nota</span>
        </button>
      </div>

      <!-- Search input -->
      <div class="relative">
        <Search class="w-3.5 h-3.5 text-slate-400 absolute left-2.5 top-1/2 -translate-y-1/2" />
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Buscar notas..."
          class="w-full pl-8 pr-7 py-1.5 bg-slate-950/60 border border-slate-800 rounded-lg text-xs text-slate-200 placeholder-slate-400 focus:outline-none focus:border-sky-500/80 focus:ring-1 focus:ring-sky-500/30 transition-all selectable"
        />
        <button
          v-if="searchQuery"
          @click="searchQuery = ''"
          class="absolute right-2 top-1/2 -translate-y-1/2 text-slate-500 hover:text-slate-300"
        >
          <X class="w-3.5 h-3.5" />
        </button>
      </div>
    </div>

    <!-- Notes list -->
    <div class="flex-1 overflow-y-auto p-2 space-y-1">
      <div
        v-if="filteredNotes.length === 0"
        class="text-center py-10 px-4 text-xs text-slate-400"
      >
        <p v-if="searchQuery">No se encontraron notas con "{{ searchQuery }}".</p>
        <p v-else>No hay notas aún. ¡Crea una para comenzar!</p>
      </div>

      <div
        v-for="note in filteredNotes"
        :key="note.id"
        @click="emit('select', note)"
        :class="[
          'p-3 rounded-lg border transition-all cursor-pointer relative group',
          activeNoteId === note.id
            ? 'bg-slate-800/90 border-sky-500/50 shadow-sm shadow-sky-950/50'
            : 'bg-slate-900/40 border-slate-800/50 hover:bg-slate-800/50 hover:border-slate-700/60',
        ]"
      >
        <div class="flex items-start justify-between gap-2">
          <div class="flex items-center gap-1.5 min-w-0 flex-1">
            <FileText class="w-3.5 h-3.5 shrink-0 text-sky-400" />
            <h4 class="text-xs font-semibold text-slate-100 truncate">
              {{ note.title || 'Sin título' }}
            </h4>
          </div>
          <button
            @click="handleDelete($event, note.id)"
            class="opacity-0 group-hover:opacity-100 p-1 text-slate-400 hover:text-rose-400 hover:bg-slate-700/60 rounded transition-all"
            title="Eliminar nota"
          >
            <Trash2 class="w-3.5 h-3.5" />
          </button>
        </div>

        <p class="text-[11px] text-slate-400 mt-1 line-clamp-2 leading-relaxed">
          {{ getSnippet(note.content) }}
        </p>

        <div class="flex items-center justify-between mt-2 pt-1.5 border-t border-slate-800/40 text-[10px] text-slate-400">
          <span>{{ formatTime(note.updated_at) }}</span>
        </div>
      </div>
    </div>
  </div>
</template>
