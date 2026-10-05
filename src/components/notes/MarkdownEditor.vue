<script setup lang="ts">
import { ref, watch, computed, nextTick, onMounted, onUnmounted } from 'vue';
import {
  Check,
  CheckSquare,
  Bold,
  Italic,
  Heading1,
  Heading2,
  List,
  Code,
  Eye,
  Edit3,
  Columns,
  Loader2,
  Trash2,
  Undo2,
  Redo2,
} from 'lucide-vue-next';
import { marked } from 'marked';
import DOMPurify from 'dompurify';
import type { Note, NoteInput } from '../../types/note';

const props = defineProps<{
  note: Note;
}>();

const emit = defineEmits<{
  (e: 'save', input: NoteInput): void;
  (e: 'delete', id: string): void;
}>();

const title = ref(props.note.title);
const content = ref(props.note.content);
const viewMode = ref<'split' | 'edit' | 'preview'>('split');
const saveStatus = ref<'saved' | 'saving'>('saved');

const textareaRef = ref<HTMLTextAreaElement | null>(null);
const previewRef = ref<HTMLDivElement | null>(null);

let saveTimeout: any = null;

// Undo / Redo History Stack
interface HistoryEntry {
  text: string;
  cursor: number;
}

const history = ref<HistoryEntry[]>([]);
const historyIndex = ref<number>(-1);
const isNavigatingHistory = ref(false);
let lastSnapshotTime = 0;
let snapshotTimer: any = null;

const canUndo = computed(() => historyIndex.value > 0);
const canRedo = computed(() => historyIndex.value >= 0 && historyIndex.value < history.value.length - 1);

function initHistory(initialText: string) {
  history.value = [
    {
      text: initialText,
      cursor: initialText.length,
    },
  ];
  historyIndex.value = 0;
  lastSnapshotTime = Date.now();
}

function recordSnapshot(cursor?: number) {
  if (isNavigatingHistory.value) return;

  const currentCursor =
    cursor !== undefined
      ? cursor
      : textareaRef.value
      ? textareaRef.value.selectionStart
      : content.value.length;

  const currentEntry = history.value[historyIndex.value];
  if (currentEntry && currentEntry.text === content.value) {
    currentEntry.cursor = currentCursor;
    return;
  }

  // Truncate redo history branch
  if (historyIndex.value < history.value.length - 1) {
    history.value = history.value.slice(0, historyIndex.value + 1);
  }

  history.value.push({
    text: content.value,
    cursor: currentCursor,
  });

  if (history.value.length > 150) {
    history.value.shift();
  } else {
    historyIndex.value++;
  }

  lastSnapshotTime = Date.now();
}

function undo() {
  if (!canUndo.value) return;
  isNavigatingHistory.value = true;

  // Save current cursor position in current history entry before moving
  if (textareaRef.value && history.value[historyIndex.value]) {
    history.value[historyIndex.value].cursor = textareaRef.value.selectionStart;
  }

  historyIndex.value--;
  const entry = history.value[historyIndex.value];
  content.value = entry.text;

  nextTick(() => {
    if (textareaRef.value) {
      textareaRef.value.setSelectionRange(entry.cursor, entry.cursor);
      textareaRef.value.focus();
    }
    isNavigatingHistory.value = false;
    triggerAutoSave();
  });
}

function redo() {
  if (!canRedo.value) return;
  isNavigatingHistory.value = true;

  historyIndex.value++;
  const entry = history.value[historyIndex.value];
  content.value = entry.text;

  nextTick(() => {
    if (textareaRef.value) {
      textareaRef.value.setSelectionRange(entry.cursor, entry.cursor);
      textareaRef.value.focus();
    }
    isNavigatingHistory.value = false;
    triggerAutoSave();
  });
}

// Watch for note prop changes (e.g. switching active note)
watch(
  () => props.note.id,
  () => {
    if (saveTimeout) clearTimeout(saveTimeout);
    if (snapshotTimer) clearTimeout(snapshotTimer);
    title.value = props.note.title;
    content.value = props.note.content;
    saveStatus.value = 'saved';
    initHistory(props.note.content);
  },
  { immediate: true }
);

// Trigger debounced auto-save
function triggerAutoSave() {
  saveStatus.value = 'saving';
  if (saveTimeout) clearTimeout(saveTimeout);
  saveTimeout = setTimeout(() => {
    emit('save', {
      id: props.note.id,
      title: title.value.trim() || 'Sin título',
      content: content.value,
      is_checklist: false,
    });
    saveStatus.value = 'saved';
  }, 350);
}

function handleTitleInput() {
  triggerAutoSave();
}

function handleContentInput(e: Event) {
  const inputEvent = e as InputEvent;
  const data = inputEvent?.data;
  const isBoundary =
    data === ' ' ||
    data === '\n' ||
    data === '.' ||
    inputEvent?.inputType?.startsWith('delete') ||
    inputEvent?.inputType === 'insertFromPaste';

  if (snapshotTimer) clearTimeout(snapshotTimer);

  if (isBoundary || Date.now() - lastSnapshotTime > 800) {
    recordSnapshot();
  } else {
    snapshotTimer = setTimeout(() => {
      recordSnapshot();
    }, 350);
  }

  triggerAutoSave();
}

// Keyboard shortcuts for textarea
function handleTextareaKeyDown(e: KeyboardEvent) {
  // Ctrl+Z or Cmd+Z -> Undo / Redo
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'z') {
    e.preventDefault();
    if (e.shiftKey) {
      redo();
    } else {
      undo();
    }
    return;
  }

  // Ctrl+Y -> Redo
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'y') {
    e.preventDefault();
    redo();
    return;
  }

  // Tab key -> 2 spaces indentation
  if (e.key === 'Tab') {
    e.preventDefault();
    insertFormat('  ');
    return;
  }
}

// Global key listener for Ctrl+Z when editor is active
function handleGlobalKeyDown(e: KeyboardEvent) {
  const activeEl = document.activeElement;
  const isEditingTextarea = activeEl === textareaRef.value;
  // If focus is not inside an input other than our editor
  if (!isEditingTextarea && activeEl?.tagName !== 'INPUT' && activeEl?.tagName !== 'TEXTAREA') {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'z') {
      e.preventDefault();
      if (e.shiftKey) {
        redo();
      } else {
        undo();
      }
    } else if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'y') {
      e.preventDefault();
      redo();
    }
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleGlobalKeyDown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleGlobalKeyDown);
});

// Markdown parser
const renderedMarkdown = computed(() => {
  if (!content.value) return '<p class="text-slate-500 italic">Escribe algo en Markdown...</p>';
  const rawHtml = marked.parse(content.value, { gfm: true, breaks: true }) as string;
  const interactiveHtml = rawHtml.replace(/<input\s+([^>]*?)disabled(?:="")?\s*([^>]*?)>/gi, '<input $1 $2>');
  return DOMPurify.sanitize(interactiveHtml);
});

// Handle interactive checklist clicks in preview
function handlePreviewClick(e: MouseEvent) {
  const target = e.target as HTMLElement;
  if (target && target.tagName === 'INPUT' && (target as HTMLInputElement).type === 'checkbox') {
    e.preventDefault();
    if (!previewRef.value) return;
    const checkboxes = Array.from(previewRef.value.querySelectorAll('input[type="checkbox"]'));
    const index = checkboxes.indexOf(target as HTMLInputElement);
    if (index >= 0) {
      toggleCheckboxInMarkdown(index);
    }
  }
}

// Flip n-th checkbox in markdown text
function toggleCheckboxInMarkdown(targetIndex: number) {
  recordSnapshot();
  const checkboxRegex = /^(\s*[-*+]\s+\[)([\sxX])(\])/gm;
  let currentIndex = 0;

  content.value = content.value.replace(checkboxRegex, (match, prefix, checkState, suffix) => {
    if (currentIndex === targetIndex) {
      const isChecked = checkState.toLowerCase() === 'x';
      const newState = isChecked ? ' ' : 'x';
      currentIndex++;
      return `${prefix}${newState}${suffix}`;
    }
    currentIndex++;
    return match;
  });

  recordSnapshot();
  triggerAutoSave();
}

// Insert formatting at cursor
function insertFormat(prefix: string, suffix: string = '') {
  if (!textareaRef.value) return;
  recordSnapshot();

  const textarea = textareaRef.value;
  const start = textarea.selectionStart;
  const end = textarea.selectionEnd;
  const selectedText = content.value.substring(start, end);
  const before = content.value.substring(0, start);
  const after = content.value.substring(end);

  content.value = `${before}${prefix}${selectedText}${suffix}${after}`;

  const newPos = start + prefix.length + selectedText.length;
  recordSnapshot(newPos);
  triggerAutoSave();

  nextTick(() => {
    textarea.focus();
    textarea.setSelectionRange(newPos, newPos);
  });
}

function insertLinePrefix(linePrefix: string) {
  if (!textareaRef.value) return;
  recordSnapshot();

  const textarea = textareaRef.value;
  const start = textarea.selectionStart;
  const before = content.value.substring(0, start);
  const lastNewline = before.lastIndexOf('\n');
  const lineStart = lastNewline === -1 ? 0 : lastNewline + 1;
  const lineEnd = content.value.indexOf('\n', start);
  const actualLineEnd = lineEnd === -1 ? content.value.length : lineEnd;

  const currentLine = content.value.substring(lineStart, actualLineEnd);
  const beforeLine = content.value.substring(0, lineStart);
  const afterLine = content.value.substring(actualLineEnd);

  content.value = `${beforeLine}${linePrefix}${currentLine}${afterLine}`;

  const newCursor = start + linePrefix.length;
  recordSnapshot(newCursor);
  triggerAutoSave();

  nextTick(() => {
    textarea.focus();
    textarea.setSelectionRange(newCursor, newCursor);
  });
}
</script>

<template>
  <div class="flex-1 h-full bg-slate-950 flex flex-col min-w-0 overflow-hidden">
    <!-- Note Toolbar & Title -->
    <div class="p-3 border-b border-slate-800/80 bg-slate-900/50 flex flex-col gap-2 shrink-0">
      <div class="flex items-center justify-between gap-3">
        <!-- Title Input -->
        <input
          v-model="title"
          @input="handleTitleInput"
          type="text"
          placeholder="Título de la nota..."
          class="text-base font-semibold bg-transparent text-slate-100 placeholder-slate-400 focus:outline-none flex-1 border-b border-transparent focus:border-sky-500/50 pb-0.5 transition-colors selectable"
        />

        <!-- Right Controls: Status & Mode & Delete -->
        <div class="flex items-center gap-2">
          <!-- Auto-save status -->
          <div class="flex items-center gap-1.5 px-2 py-1 rounded bg-slate-900 border border-slate-800 text-[11px] text-slate-400 select-none">
            <template v-if="saveStatus === 'saved'">
              <Check class="w-3 h-3 text-emerald-400" />
              <span>Guardado</span>
            </template>
            <template v-else>
              <Loader2 class="w-3 h-3 text-sky-400 animate-spin" />
              <span>Guardando...</span>
            </template>
          </div>

          <!-- View Mode Toggles -->
          <div class="flex items-center p-0.5 bg-slate-900 border border-slate-800 rounded-lg">
            <button
              @click="viewMode = 'edit'"
              :class="[
                'p-1.5 rounded transition-colors cursor-pointer',
                viewMode === 'edit' ? 'bg-slate-800 text-sky-400' : 'text-slate-400 hover:text-slate-200',
              ]"
              title="Solo editor"
            >
              <Edit3 class="w-3.5 h-3.5" />
            </button>
            <button
              @click="viewMode = 'split'"
              :class="[
                'p-1.5 rounded transition-colors cursor-pointer',
                viewMode === 'split' ? 'bg-slate-800 text-sky-400' : 'text-slate-400 hover:text-slate-200',
              ]"
              title="Vista dividida"
            >
              <Columns class="w-3.5 h-3.5" />
            </button>
            <button
              @click="viewMode = 'preview'"
              :class="[
                'p-1.5 rounded transition-colors cursor-pointer',
                viewMode === 'preview' ? 'bg-slate-800 text-sky-400' : 'text-slate-400 hover:text-slate-200',
              ]"
              title="Solo vista previa"
            >
              <Eye class="w-3.5 h-3.5" />
            </button>
          </div>

          <!-- Delete button -->
          <button
            @click="emit('delete', note.id)"
            class="p-1.5 text-slate-400 hover:text-rose-400 hover:bg-slate-800/80 rounded-lg transition-colors cursor-pointer"
            title="Eliminar nota"
          >
            <Trash2 class="w-4 h-4" />
          </button>
        </div>
      </div>

      <!-- Format Buttons & History (Undo/Redo) -->
      <div class="flex items-center justify-between text-xs text-slate-400 pt-1">
        <div class="flex items-center gap-1">
          <!-- Undo / Redo buttons -->
          <button
            @click="undo"
            :disabled="!canUndo"
            class="p-1.5 rounded hover:bg-slate-800 hover:text-slate-200 disabled:opacity-30 disabled:hover:bg-transparent disabled:hover:text-slate-400 transition-colors cursor-pointer"
            title="Deshacer (Ctrl+Z)"
          >
            <Undo2 class="w-3.5 h-3.5" />
          </button>
          <button
            @click="redo"
            :disabled="!canRedo"
            class="p-1.5 rounded hover:bg-slate-800 hover:text-slate-200 disabled:opacity-30 disabled:hover:bg-transparent disabled:hover:text-slate-400 transition-colors cursor-pointer"
            title="Rehacer (Ctrl+Y o Ctrl+Shift+Z)"
          >
            <Redo2 class="w-3.5 h-3.5" />
          </button>

          <span class="w-px h-4 bg-slate-800 mx-1"></span>

          <!-- Formatting shortcuts -->
          <button
            @click="insertFormat('**', '**')"
            class="p-1.5 rounded hover:bg-slate-800 hover:text-slate-200 transition-colors cursor-pointer"
            title="Negrita (**)"
          >
            <Bold class="w-3.5 h-3.5" />
          </button>
          <button
            @click="insertFormat('*', '*')"
            class="p-1.5 rounded hover:bg-slate-800 hover:text-slate-200 transition-colors cursor-pointer"
            title="Cursiva (*)"
          >
            <Italic class="w-3.5 h-3.5" />
          </button>
          <button
            @click="insertLinePrefix('# ')"
            class="p-1.5 rounded hover:bg-slate-800 hover:text-slate-200 transition-colors cursor-pointer"
            title="Título 1 (#)"
          >
            <Heading1 class="w-3.5 h-3.5" />
          </button>
          <button
            @click="insertLinePrefix('## ')"
            class="p-1.5 rounded hover:bg-slate-800 hover:text-slate-200 transition-colors cursor-pointer"
            title="Título 2 (##)"
          >
            <Heading2 class="w-3.5 h-3.5" />
          </button>
          <button
            @click="insertLinePrefix('- ')"
            class="p-1.5 rounded hover:bg-slate-800 hover:text-slate-200 transition-colors cursor-pointer"
            title="Lista con viñetas"
          >
            <List class="w-3.5 h-3.5" />
          </button>
          <button
            @click="insertLinePrefix('- [ ] ')"
            class="p-1.5 rounded hover:bg-slate-800 hover:text-slate-200 transition-colors cursor-pointer"
            title="Añadir ítem de tarea interactiva (- [ ])"
          >
            <CheckSquare class="w-3.5 h-3.5 text-emerald-400" />
          </button>
          <button
            @click="insertFormat('```\n', '\n```')"
            class="p-1.5 rounded hover:bg-slate-800 hover:text-slate-200 transition-colors cursor-pointer"
            title="Bloque de código"
          >
            <Code class="w-3.5 h-3.5" />
          </button>
        </div>

        <div class="text-[11px] text-slate-500 font-mono hidden sm:block">
          Markdown + Checklists
        </div>
      </div>
    </div>

    <!-- Main Workspace (Editor / Preview) -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Raw Markdown Textarea -->
      <div
        v-show="viewMode === 'edit' || viewMode === 'split'"
        :class="[
          'h-full flex flex-col min-w-0 bg-slate-950 p-4',
          viewMode === 'split' ? 'w-1/2 border-r border-slate-800' : 'w-full',
        ]"
      >
        <textarea
          ref="textareaRef"
          v-model="content"
          @input="handleContentInput"
          @keydown="handleTextareaKeyDown"
          placeholder="Escribe tus notas en Markdown aquí... (usa - [ ] para tareas interactivas, Ctrl+Z para deshacer, Tab para sangría)"
          class="w-full h-full bg-transparent text-slate-200 placeholder-slate-500 font-mono text-sm leading-relaxed resize-none focus:outline-none selectable"
          spellcheck="false"
        ></textarea>
      </div>

      <!-- Rendered Markdown Preview -->
      <div
        v-show="viewMode === 'preview' || viewMode === 'split'"
        :class="[
          'h-full overflow-y-auto p-6 bg-slate-900/30 min-w-0',
          viewMode === 'split' ? 'w-1/2' : 'w-full',
        ]"
      >
        <div
          ref="previewRef"
          @click="handlePreviewClick"
          class="markdown-body selectable max-w-none text-slate-200"
          v-html="renderedMarkdown"
        ></div>
      </div>
    </div>
  </div>
</template>
