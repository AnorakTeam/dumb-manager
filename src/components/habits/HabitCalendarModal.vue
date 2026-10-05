<script setup lang="ts">
import Modal from '../common/Modal.vue';
import MonthHeatmap from './MonthHeatmap.vue';
import type { HabitProgress, HabitLog } from '../../types/habit';

const props = defineProps<{
  isOpen: boolean;
  habit: HabitProgress;
  logs: HabitLog[];
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'change-month', payload: { habitId: string; year: number; month: number }): void;
}>();

function handleMonthChange(payload: { year: number; month: number }) {
  emit('change-month', {
    habitId: props.habit.id,
    year: payload.year,
    month: payload.month,
  });
}
</script>

<template>
  <Modal
    :is-open="isOpen"
    :title="`Auditoría Mensual: ${habit.title}`"
    max-width="max-w-xl"
    @close="emit('close')"
  >
    <MonthHeatmap
      :habit="habit"
      :logs="logs"
      @change-month="handleMonthChange"
    />

    <template #footer>
      <button
        @click="emit('close')"
        class="px-4 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-medium transition-colors"
      >
        Cerrar
      </button>
    </template>
  </Modal>
</template>
