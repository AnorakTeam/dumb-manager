<script setup lang="ts">
import { ref } from 'vue';
import { Plus } from 'lucide-vue-next';
import Modal from '../common/Modal.vue';

defineProps<{
  isOpen: boolean;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'create', data: { title: string; dailyTarget: number; color: string }): void;
}>();

const title = ref('');
const dailyTarget = ref(1);
const selectedColor = ref('#38bdf8');

const colorPresets = [
  { name: 'Azul Cielo', value: '#38bdf8' },
  { name: 'Esmeralda', value: '#10b981' },
  { name: 'Violeta', value: '#a855f7' },
  { name: 'Ámbar', value: '#f59e0b' },
  { name: 'Rosa', value: '#f43f5e' },
  { name: 'Cian', value: '#06b6d4' },
  { name: 'Índigo', value: '#6366f1' },
];

const suggestions = [
  { title: 'Beber 2L de agua (vasos)', target: 8, color: '#38bdf8' },
  { title: 'Cepillarse los dientes', target: 3, color: '#06b6d4' },
  { title: 'Hacer ejercicio / Gimnasio', target: 1, color: '#10b981' },
  { title: 'Leer 20 páginas', target: 1, color: '#a855f7' },
  { title: 'Meditar', target: 1, color: '#6366f1' },
  { title: 'Tomar vitaminas/medicamento', target: 2, color: '#f59e0b' },
];

function handleSubmit() {
  if (!title.value.trim() || dailyTarget.value < 1) return;
  emit('create', {
    title: title.value.trim(),
    dailyTarget: dailyTarget.value,
    color: selectedColor.value,
  });
  title.value = '';
  dailyTarget.value = 1;
}

function selectSuggestion(s: { title: string; target: number; color: string }) {
  title.value = s.title;
  dailyTarget.value = s.target;
  selectedColor.value = s.color;
}
</script>

<template>
  <Modal
    :is-open="isOpen"
    title="Crear Hábito Cuantitativo"
    max-width="max-w-md"
    @close="emit('close')"
  >
    <form @submit.prevent="handleSubmit" class="space-y-4">
      <!-- Title -->
      <div>
        <label class="block text-xs font-semibold text-slate-300 mb-1.5">
          Nombre del hábito:
        </label>
        <input
          v-model="title"
          type="text"
          placeholder="ej. Tomar agua, Hacer flexiones..."
          class="w-full px-3 py-2 bg-slate-950 border border-slate-800 rounded-lg text-sm text-slate-100 placeholder-slate-500 focus:outline-none focus:border-sky-500/80 focus:ring-1 focus:ring-sky-500/30 transition-all selectable"
          autofocus
          required
        />
      </div>

      <!-- Target -->
      <div>
        <label class="block text-xs font-semibold text-slate-300 mb-1.5">
          Meta diaria requerida para considerar completado el día:
        </label>
        <div class="flex items-center gap-3">
          <input
            v-model.number="dailyTarget"
            type="number"
            min="1"
            max="100"
            class="w-24 px-3 py-2 bg-slate-950 border border-slate-800 rounded-lg text-sm text-slate-100 focus:outline-none focus:border-sky-500/80 focus:ring-1 focus:ring-sky-500/30 font-mono text-center selectable"
            required
          />
          <span class="text-xs text-slate-400">
            veces por día (alcanzar o superar esta cifra marca el día completado)
          </span>
        </div>
      </div>

      <!-- Color picker -->
      <div>
        <label class="block text-xs font-semibold text-slate-300 mb-2">
          Color del hábito (para el heatmap y progreso):
        </label>
        <div class="flex items-center gap-2">
          <button
            v-for="c in colorPresets"
            :key="c.value"
            type="button"
            @click="selectedColor = c.value"
            :style="{ backgroundColor: c.value }"
            :class="[
              'w-7 h-7 rounded-full transition-transform cursor-pointer relative',
              selectedColor === c.value ? 'ring-2 ring-white ring-offset-2 ring-offset-slate-900 scale-110' : 'hover:scale-105 opacity-80',
            ]"
            :title="c.name"
          ></button>
        </div>
      </div>

      <!-- Suggestions -->
      <div>
        <span class="block text-[11px] font-medium text-slate-400 mb-2">
          Plantillas frecuentes:
        </span>
        <div class="flex flex-wrap gap-1.5">
          <button
            v-for="s in suggestions"
            :key="s.title"
            type="button"
            @click="selectSuggestion(s)"
            class="px-2.5 py-1 rounded-md bg-slate-950/70 hover:bg-slate-800 text-[11px] text-slate-300 border border-slate-800/80 transition-colors"
          >
            {{ s.title }} ({{ s.target }}/día)
          </button>
        </div>
      </div>
    </form>

    <template #footer>
      <button
        type="button"
        @click="emit('close')"
        class="px-4 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 text-xs font-medium transition-colors"
      >
        Cancelar
      </button>
      <button
        type="button"
        @click="handleSubmit"
        :disabled="!title.trim() || dailyTarget < 1"
        class="flex items-center gap-1.5 px-4 py-1.5 rounded-lg bg-sky-500 hover:bg-sky-400 disabled:opacity-50 text-slate-950 text-xs font-bold transition-colors cursor-pointer"
      >
        <Plus class="w-4 h-4" />
        <span>Crear Hábito</span>
      </button>
    </template>
  </Modal>
</template>
