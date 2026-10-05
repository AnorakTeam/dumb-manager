<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { ChevronLeft, ChevronRight, Check } from 'lucide-vue-next';
import dayjs from 'dayjs';
import 'dayjs/locale/es';
import type { HabitProgress, HabitLog } from '../../types/habit';

dayjs.locale('es');

const props = defineProps<{
  habit: HabitProgress;
  logs: HabitLog[];
  initialYear?: number;
  initialMonth?: number; // 1-12
}>();

const emit = defineEmits<{
  (e: 'change-month', payload: { year: number; month: number }): void;
}>();

const currentDate = ref(dayjs());
const selectedYear = ref(props.initialYear ?? currentDate.value.year());
const selectedMonth = ref(props.initialMonth ?? currentDate.value.month() + 1); // 1-indexed

watch(
  [selectedYear, selectedMonth],
  ([y, m]) => {
    emit('change-month', { year: y, month: m });
  },
  { immediate: true }
);

function prevMonth() {
  const d = dayjs(`${selectedYear.value}-${selectedMonth.value}-01`).subtract(1, 'month');
  selectedYear.value = d.year();
  selectedMonth.value = d.month() + 1;
}

function nextMonth() {
  const d = dayjs(`${selectedYear.value}-${selectedMonth.value}-01`).add(1, 'month');
  selectedYear.value = d.year();
  selectedMonth.value = d.month() + 1;
}

const monthName = computed(() => {
  const d = dayjs(`${selectedYear.value}-${selectedMonth.value}-01`);
  const name = d.format('MMMM YYYY');
  return name.charAt(0).toUpperCase() + name.slice(1);
});

// Logs map for fast lookup by "YYYY-MM-DD"
const logsMap = computed(() => {
  const map = new Map<string, number>();
  for (const l of props.logs) {
    map.set(l.date, l.current_count);
  }
  return map;
});

// Weekday headers
const weekDays = ['Lun', 'Mar', 'Mié', 'Jue', 'Vie', 'Sáb', 'Dom'];

// Calendar grid calculations
const calendarCells = computed(() => {
  const firstDay = dayjs(`${selectedYear.value}-${selectedMonth.value}-01`);
  const daysInMonth = firstDay.daysInMonth();
  
  // Day of week: 0 (Sun) to 6 (Sat). We want Mon (0) to Sun (6)
  let startOffset = firstDay.day() - 1;
  if (startOffset === -1) startOffset = 6; // Sunday becomes 6

  const cells: Array<{
    dayNumber?: number;
    dateStr?: string;
    count: number;
    status: 'empty-pad' | 'completed' | 'partial' | 'zero' | 'future';
    isToday: boolean;
  }> = [];

  // Padding cells before first day
  for (let i = 0; i < startOffset; i++) {
    cells.push({
      status: 'empty-pad',
      count: 0,
      isToday: false,
    });
  }

  const todayStr = dayjs().format('YYYY-MM-DD');

  // Days of current month
  for (let day = 1; day <= daysInMonth; day++) {
    const dateStr = `${selectedYear.value}-${String(selectedMonth.value).padStart(2, '0')}-${String(day).padStart(2, '0')}`;
    const count = logsMap.value.get(dateStr) ?? 0;
    const isToday = dateStr === todayStr;
    const isFuture = dayjs(dateStr).isAfter(dayjs(), 'day');

    let status: 'completed' | 'partial' | 'zero' | 'future' = 'zero';
    if (count >= props.habit.daily_target) {
      status = 'completed';
    } else if (count > 0) {
      status = 'partial';
    } else if (isFuture) {
      status = 'future';
    }

    cells.push({
      dayNumber: day,
      dateStr,
      count,
      status,
      isToday,
    });
  }

  return cells;
});

// Summary stats for this month
const stats = computed(() => {
  const daysInMonth = dayjs(`${selectedYear.value}-${selectedMonth.value}-01`).daysInMonth();
  let completedDays = 0;
  let totalCount = 0;

  for (let d = 1; d <= daysInMonth; d++) {
    const dateStr = `${selectedYear.value}-${String(selectedMonth.value).padStart(2, '0')}-${String(d).padStart(2, '0')}`;
    const count = logsMap.value.get(dateStr) ?? 0;
    totalCount += count;
    if (count >= props.habit.daily_target) {
      completedDays++;
    }
  }

  // Calculate consistency rate against passed days in current month or full month
  const isCurrentMonth =
    selectedYear.value === currentDate.value.year() &&
    selectedMonth.value === currentDate.value.month() + 1;

  const denominator = isCurrentMonth ? Math.min(daysInMonth, currentDate.value.date()) : daysInMonth;
  const consistencyRate = denominator > 0 ? Math.round((completedDays / denominator) * 100) : 0;

  return {
    completedDays,
    daysInMonth,
    totalCount,
    consistencyRate,
  };
});
</script>

<template>
  <div class="space-y-4">
    <!-- Month Navigation Header -->
    <div class="flex items-center justify-between px-2 py-1 bg-slate-950/60 rounded-xl border border-slate-800/80">
      <button
        @click="prevMonth"
        class="p-1.5 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800 transition-colors cursor-pointer"
        title="Mes anterior"
      >
        <ChevronLeft class="w-5 h-5" />
      </button>

      <div class="text-sm font-bold text-slate-100 flex items-center gap-2">
        <span>{{ monthName }}</span>
        <span
          class="w-2.5 h-2.5 rounded-full inline-block"
          :style="{ backgroundColor: habit.color || '#3b82f6' }"
        ></span>
      </div>

      <button
        @click="nextMonth"
        class="p-1.5 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800 transition-colors cursor-pointer"
        title="Mes siguiente"
      >
        <ChevronRight class="w-5 h-5" />
      </button>
    </div>

    <!-- Monthly Statistics Pill Cards -->
    <div class="grid grid-cols-3 gap-2.5">
      <div class="p-3 rounded-lg bg-slate-950/70 border border-slate-800/80 text-center">
        <span class="text-[11px] text-slate-400 block font-medium">Días Completados</span>
        <span class="text-base font-bold text-slate-100 font-mono mt-0.5 inline-block">
          {{ stats.completedDays }} <span class="text-xs text-slate-400 font-normal">/ {{ stats.daysInMonth }}</span>
        </span>
      </div>

      <div class="p-3 rounded-lg bg-slate-950/70 border border-slate-800/80 text-center">
        <span class="text-[11px] text-slate-400 block font-medium">Consistencia</span>
        <span
          class="text-base font-bold font-mono mt-0.5 inline-block"
          :style="{ color: stats.consistencyRate >= 70 ? (habit.color || '#3b82f6') : '#cbd5e1' }"
        >
          {{ stats.consistencyRate }}%
        </span>
      </div>

      <div class="p-3 rounded-lg bg-slate-950/70 border border-slate-800/80 text-center">
        <span class="text-[11px] text-slate-400 block font-medium">Total Conteo</span>
        <span class="text-base font-bold text-slate-100 font-mono mt-0.5 inline-block">
          {{ stats.totalCount }}
        </span>
      </div>
    </div>

    <!-- Calendar Grid -->
    <div class="p-3 rounded-xl bg-slate-950/70 border border-slate-800/80">
      <!-- Weekday headers -->
      <div class="grid grid-cols-7 gap-1.5 mb-2 text-center text-[11px] font-semibold text-slate-400">
        <span v-for="wd in weekDays" :key="wd">{{ wd }}</span>
      </div>

      <!-- Days Grid -->
      <div class="grid grid-cols-7 gap-1.5">
        <div
          v-for="(cell, idx) in calendarCells"
          :key="idx"
          :class="[
            'aspect-square rounded-lg flex flex-col items-center justify-center p-1 relative text-xs transition-all select-none',
            cell.status === 'empty-pad' ? 'opacity-0 pointer-events-none' : '',
            cell.status === 'completed'
              ? 'text-slate-950 font-bold shadow-xs'
              : cell.status === 'partial'
              ? 'bg-slate-900 border text-slate-200 font-medium'
              : cell.status === 'future'
              ? 'bg-slate-900/30 text-slate-400 border border-slate-900'
              : 'bg-slate-900/60 text-slate-400 border border-slate-800/40 hover:border-slate-700',
            cell.isToday ? 'ring-2 ring-sky-400 ring-offset-1 ring-offset-slate-950' : '',
          ]"
          :style="
            cell.status === 'completed'
              ? { backgroundColor: habit.color || '#3b82f6', color: '#090d16' }
              : cell.status === 'partial'
              ? { borderColor: habit.color || '#3b82f6', backgroundColor: `${habit.color}18` }
              : {}
          "
        >
          <template v-if="cell.dayNumber">
            <span class="text-[11px] leading-none">{{ cell.dayNumber }}</span>

            <!-- Badge / Icon indicator inside cell -->
            <div class="mt-0.5">
              <template v-if="cell.status === 'completed'">
                <Check class="w-3 h-3 stroke-[3]" />
              </template>
              <template v-else-if="cell.status === 'partial'">
                <span class="text-[9px] font-mono leading-none" :style="{ color: habit.color }">
                  {{ cell.count }}/{{ habit.daily_target }}
                </span>
              </template>
            </div>
          </template>
        </div>
      </div>
    </div>

    <!-- Legend -->
    <div class="flex items-center justify-center gap-4 text-[11px] text-slate-400 pt-1">
      <div class="flex items-center gap-1.5">
        <span
          class="w-3 h-3 rounded"
          :style="{ backgroundColor: habit.color || '#3b82f6' }"
        ></span>
        <span>Completado (&ge;{{ habit.daily_target }})</span>
      </div>
      <div class="flex items-center gap-1.5">
        <span
          class="w-3 h-3 rounded border"
          :style="{ borderColor: habit.color || '#3b82f6', backgroundColor: `${habit.color}20` }"
        ></span>
        <span>Progreso parcial</span>
      </div>
      <div class="flex items-center gap-1.5">
        <span class="w-3 h-3 rounded bg-slate-900 border border-slate-800"></span>
        <span>Sin registro</span>
      </div>
    </div>
  </div>
</template>
