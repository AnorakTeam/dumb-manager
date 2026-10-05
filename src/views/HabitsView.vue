<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { Plus, CalendarCheck2, Sparkles } from 'lucide-vue-next';
import dayjs from 'dayjs';
import { useHabits } from '../composables/useHabits';
import { useConfirm } from '../composables/useConfirm';
import { toast } from '../composables/useToast';
import HabitTrackerRow from '../components/habits/HabitTrackerRow.vue';
import HabitCreateModal from '../components/habits/HabitCreateModal.vue';
import HabitCalendarModal from '../components/habits/HabitCalendarModal.vue';
import type { HabitProgress, HabitLog } from '../types/habit';

const {
  habits,
  loading,
  fetchHabits,
  createHabit,
  incrementHabit,
  decrementHabit,
  getMonthLogs,
  deleteHabit,
} = useHabits();

const { confirm } = useConfirm();

const isCreateModalOpen = ref(false);

const isCalendarModalOpen = ref(false);
const activeCalendarHabit = ref<HabitProgress | null>(null);
const activeCalendarLogs = ref<HabitLog[]>([]);

onMounted(() => {
  fetchHabits();
});

function getTodayStr() {
  return dayjs().format('YYYY-MM-DD');
}

async function handleIncrement(habitId: string) {
  await incrementHabit(habitId, getTodayStr());
  // If calendar modal is open for this habit, refresh month logs
  if (isCalendarModalOpen.value && activeCalendarHabit.value?.id === habitId) {
    const now = dayjs();
    activeCalendarLogs.value = await getMonthLogs(habitId, now.year(), now.month() + 1);
  }
}

async function handleDecrement(habitId: string) {
  await decrementHabit(habitId, getTodayStr());
  if (isCalendarModalOpen.value && activeCalendarHabit.value?.id === habitId) {
    const now = dayjs();
    activeCalendarLogs.value = await getMonthLogs(habitId, now.year(), now.month() + 1);
  }
}

async function handleOpenCalendar(habitId: string) {
  const found = habits.value.find((h) => h.id === habitId);
  if (found) {
    activeCalendarHabit.value = found;
    const now = dayjs();
    activeCalendarLogs.value = await getMonthLogs(habitId, now.year(), now.month() + 1);
    isCalendarModalOpen.value = true;
  }
}

async function handleChangeMonth(payload: { habitId: string; year: number; month: number }) {
  activeCalendarLogs.value = await getMonthLogs(payload.habitId, payload.year, payload.month);
}

async function handleCreate(data: { title: string; dailyTarget: number; color: string }) {
  await createHabit(data.title, data.dailyTarget, data.color);
  isCreateModalOpen.value = false;
  toast.success(`Hábito "${data.title}" creado`);
}

async function handleDelete(habitId: string) {
  const habit = habits.value.find((h) => h.id === habitId);
  const title = habit ? habit.title : 'este hábito';
  const ok = await confirm({
    title: '¿Eliminar hábito?',
    message: `¿Estás seguro de eliminar el hábito "${title}"? Se borrarán de forma definitiva todos los registros diarios asociados a él.`,
    confirmText: 'Eliminar hábito',
    cancelText: 'Cancelar',
    type: 'danger',
  });
  if (ok) {
    await deleteHabit(habitId);
    toast.success('Hábito eliminado');
  }
}
</script>

<template>
  <div class="h-full w-full flex flex-col bg-slate-950 overflow-hidden">
    <!-- Top Header Bar -->
    <header class="p-6 border-b border-slate-800/80 bg-slate-900/40 flex items-center justify-between shrink-0">
      <div>
        <div class="flex items-center gap-2">
          <CalendarCheck2 class="w-5 h-5 text-sky-400" />
          <h2 class="text-lg font-bold text-white tracking-tight">Hábitos Diarios y Metas</h2>
        </div>
        <p class="text-xs text-slate-400 mt-0.5">
          Registra tus progresos diarios con metas numéricas estrictas y audita tu constancia mensual.
        </p>
      </div>

      <button
        @click="isCreateModalOpen = true"
        class="flex items-center gap-2 px-3.5 py-2 bg-sky-500 hover:bg-sky-400 text-slate-950 font-bold text-xs rounded-lg shadow-sm shadow-sky-950 transition-colors cursor-pointer"
      >
        <Plus class="w-4 h-4" />
        <span>Nuevo Hábito</span>
      </button>
    </header>

    <!-- Main Content List -->
    <main class="flex-1 overflow-y-auto p-6">
      <div v-if="habits.length === 0 && !loading" class="h-full flex flex-col items-center justify-center text-center p-8 select-none">
        <div class="relative mb-4">
          <img
            src="/logo-avatar.png"
            alt="dumb-manager"
            class="w-60 h-60 rounded-2xl object-cover border border-slate-800 shadow-xl shadow-sky-500/10"
          />
        </div>
        <h3 class="text-base font-semibold text-slate-200">No hay ningún hábito configurado</h3>
        <p class="text-xs text-slate-400 mt-1 max-w-sm">
          Define metas cuantitativas como "Beber 8 vasos de agua", "3 cepillados al día" o "1 sesión de lectura" para auditar en el calendario.
        </p>
        <button
          @click="isCreateModalOpen = true"
          class="mt-4 flex items-center gap-2 px-4 py-2 bg-sky-500 hover:bg-sky-400 text-slate-950 font-bold text-xs rounded-lg shadow-sm transition-colors cursor-pointer"
        >
          <Sparkles class="w-4 h-4" />
          <span>Crear Primer Hábito</span>
        </button>
      </div>

      <div v-else class="space-y-3 max-w-4xl mx-auto">
        <HabitTrackerRow
          v-for="h in habits"
          :key="h.id"
          :habit="h"
          @increment="handleIncrement"
          @decrement="handleDecrement"
          @calendar="handleOpenCalendar"
          @delete="handleDelete"
        />
      </div>
    </main>

    <!-- Modals -->
    <HabitCreateModal
      :is-open="isCreateModalOpen"
      @close="isCreateModalOpen = false"
      @create="handleCreate"
    />

    <HabitCalendarModal
      v-if="activeCalendarHabit"
      :is-open="isCalendarModalOpen"
      :habit="activeCalendarHabit"
      :logs="activeCalendarLogs"
      @close="isCalendarModalOpen = false"
      @change-month="handleChangeMonth"
    />
  </div>
</template>
