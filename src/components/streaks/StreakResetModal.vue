<script setup lang="ts">
import { ref } from 'vue';
import { RotateCcw } from 'lucide-vue-next';
import Modal from '../common/Modal.vue';

const props = defineProps<{
  isOpen: boolean;
  streakTitle: string;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'confirm', reason: string): void;
}>();

const reason = ref('');

function handleConfirm() {
  emit('confirm', reason.value.trim());
  reason.value = '';
}
</script>

<template>
  <Modal
    :is-open="isOpen"
    :title="`Reiniciar Racha: ${streakTitle}`"
    max-width="max-w-md"
    @close="emit('close')"
  >
    <div class="space-y-4">
      <p class="text-xs text-slate-300 leading-relaxed">
        Se registrará un nuevo evento de reinicio con la fecha y hora actual exacta. El contador volverá a 0 días.
      </p>

      <div>
        <label class="block text-xs font-semibold text-slate-300 mb-1.5">
          Motivo o nota (opcional):
        </label>
        <textarea
          v-model="reason"
          placeholder="ej. Tuve un desliz en una reunión social..."
          rows="3"
          class="w-full px-3 py-2 bg-slate-950 border border-slate-800 rounded-lg text-xs text-slate-100 placeholder-slate-500 focus:outline-none focus:border-rose-500/80 focus:ring-1 focus:ring-rose-500/30 transition-all resize-none selectable"
          autofocus
        ></textarea>
      </div>
    </div>

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
        @click="handleConfirm"
        class="flex items-center gap-1.5 px-4 py-1.5 rounded-lg bg-rose-600 hover:bg-rose-500 text-white text-xs font-bold transition-colors cursor-pointer shadow-sm shadow-rose-950"
      >
        <RotateCcw class="w-3.5 h-3.5" />
        <span>Confirmar Reinicio</span>
      </button>
    </template>
  </Modal>
</template>
