<script setup lang="ts">
import { ref } from 'vue';
import { Plus } from 'lucide-vue-next';
import Modal from '../common/Modal.vue';

defineProps<{
  isOpen: boolean;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'create', title: string): void;
}>();

const title = ref('');

const suggestions = [
  'Comida chatarra',
  'Comprar cosas innecesarias',
  'Redes sociales sin control',
  'Fumar / Vapear',
  'Alcohol',
  'Dormir tarde',
  'Procrastinar',
  'Bebidas azucaradas',
];

function handleSubmit() {
  if (!title.value.trim()) return;
  emit('create', title.value.trim());
  title.value = '';
}

function selectSuggestion(s: string) {
  title.value = s;
}
</script>

<template>
  <Modal
    :is-open="isOpen"
    title="Nueva Racha (Days Since)"
    max-width="max-w-md"
    @close="emit('close')"
  >
    <form @submit.prevent="handleSubmit" class="space-y-4">
      <div>
        <label class="block text-xs font-semibold text-slate-300 mb-1.5">
          ¿Qué hábito o comportamiento quieres contabilizar?
        </label>
        <input
          v-model="title"
          type="text"
          placeholder="ej. Comida rápida, Fumar, etc."
          class="w-full px-3 py-2 bg-slate-950 border border-slate-800 rounded-lg text-sm text-slate-100 placeholder-slate-500 focus:outline-none focus:border-amber-500/80 focus:ring-1 focus:ring-amber-500/30 transition-all selectable"
          autofocus
          required
        />
      </div>

      <!-- Quick suggestions -->
      <div>
        <span class="block text-[11px] font-medium text-slate-400 mb-2">
          Sugerencias rápidas:
        </span>
        <div class="flex flex-wrap gap-1.5">
          <button
            v-for="s in suggestions"
            :key="s"
            type="button"
            @click="selectSuggestion(s)"
            class="px-2.5 py-1 rounded-md bg-slate-950/70 hover:bg-slate-800 text-[11px] text-slate-300 border border-slate-800/80 transition-colors"
          >
            {{ s }}
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
        :disabled="!title.trim()"
        class="flex items-center gap-1.5 px-4 py-1.5 rounded-lg bg-amber-500 hover:bg-amber-400 disabled:opacity-50 text-slate-950 text-xs font-bold transition-colors cursor-pointer"
      >
        <Plus class="w-4 h-4" />
        <span>Crear Contador</span>
      </button>
    </template>
  </Modal>
</template>
