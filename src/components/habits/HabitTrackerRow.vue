<script setup lang="ts">
import { computed } from 'vue';
import { Plus, Minus, Check, Calendar, Trash2 } from 'lucide-vue-next';
import type { HabitProgress } from '../../types/habit';

const props = defineProps<{
  habit: HabitProgress;
}>();

const emit = defineEmits<{
  (e: 'increment', id: string): void;
  (e: 'decrement', id: string): void;
  (e: 'calendar', id: string): void;
  (e: 'delete', id: string): void;
}>();

const percentage = computed(() => {
  if (props.habit.daily_target <= 0) return 0;
  return Math.min(100, Math.round((props.habit.today_count / props.habit.daily_target) * 100));
});
</script>

<template>
  <div
    :class="[
      'p-4 rounded-xl border transition-all flex flex-col md:flex-row md:items-center justify-between gap-4 group relative overflow-hidden',
      habit.is_completed
        ? 'bg-slate-900/90 border-slate-700/80 shadow-md shadow-black/40'
        : 'bg-slate-900/50 border-slate-800/80 hover:border-slate-700/60',
    ]"
  >
    <!-- Left Accent Border according to custom habit color -->
    <div
      class="absolute left-0 top-0 bottom-0 w-1.5 transition-colors"
      :style="{ backgroundColor: habit.color || '#3b82f6' }"
    ></div>

    <!-- Title & Target info -->
    <div class="flex items-center gap-3.5 pl-2 min-w-0">
      <div
        class="w-3.5 h-3.5 rounded-full shrink-0 shadow-sm"
        :style="{ backgroundColor: habit.color || '#3b82f6' }"
      ></div>
      <div class="min-w-0">
        <div class="flex items-center gap-2">
          <h3 class="text-sm font-bold text-slate-100 truncate group-hover:text-white transition-colors">
            {{ habit.title }}
          </h3>
          <span
            v-if="habit.is_completed"
            class="flex items-center gap-1 px-2 py-0.5 rounded-full bg-emerald-500/10 text-emerald-400 text-[10px] font-bold border border-emerald-500/30 shrink-0"
          >
            <Check class="w-3 h-3" />
            <span>Completado</span>
          </span>
        </div>

        <div class="flex items-center gap-2 text-xs text-slate-400 mt-1">
          <span>Meta: <strong class="text-slate-200">{{ habit.daily_target }}</strong> al día</span>
          <span>•</span>
          <span class="font-mono text-[11px] text-slate-400">{{ percentage }}% progreso</span>
        </div>
      </div>
    </div>

    <!-- Center Progress Bar (on desktop) -->
    <div class="hidden lg:block w-48 mx-4">
      <div class="h-2 w-full bg-slate-950 rounded-full overflow-hidden border border-slate-800/60">
        <div
          class="h-full rounded-full transition-all duration-300"
          :style="{
            width: `${percentage}%`,
            backgroundColor: habit.color || '#3b82f6',
          }"
        ></div>
      </div>
    </div>

    <!-- Right Controls: Counter and Actions -->
    <div class="flex items-center justify-between md:justify-end gap-3 pl-2 md:pl-0 shrink-0">
      <!-- Quick Counter +/- -->
      <div class="flex items-center gap-1.5 bg-slate-950/80 p-1 rounded-lg border border-slate-800">
        <button
          @click="emit('decrement', habit.id)"
          :disabled="habit.today_count <= 0"
          class="w-7 h-7 rounded-md flex items-center justify-center text-slate-400 hover:text-white hover:bg-slate-800 disabled:opacity-30 disabled:hover:bg-transparent disabled:hover:text-slate-400 transition-colors"
          title="Disminuir"
        >
          <Minus class="w-3.5 h-3.5" />
        </button>

        <div class="min-w-[4.5rem] text-center font-mono text-xs font-bold text-slate-100 px-1">
          <span :style="{ color: habit.today_count >= habit.daily_target ? habit.color : '#f1f5f9' }">
            {{ habit.today_count }}
          </span>
          <span class="text-slate-500"> / </span>
          <span class="text-slate-400">{{ habit.daily_target }}</span>
        </div>

        <button
          @click="emit('increment', habit.id)"
          class="w-7 h-7 rounded-md flex items-center justify-center text-slate-200 hover:text-white hover:bg-slate-800 transition-colors"
          :style="{ color: habit.color }"
          title="Incrementar"
        >
          <Plus class="w-4 h-4 font-bold" />
        </button>
      </div>

      <!-- Calendar & Delete Buttons -->
      <div class="flex items-center gap-1">
        <button
          @click="emit('calendar', habit.id)"
          class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg bg-slate-800/60 hover:bg-slate-800 text-slate-300 hover:text-white text-xs font-medium border border-slate-700/50 transition-colors"
          title="Ver calendario y heatmap mensual"
        >
          <Calendar class="w-3.5 h-3.5 text-sky-400" />
          <span class="hidden sm:inline">Calendario</span>
        </button>

        <button
          @click="emit('delete', habit.id)"
          class="opacity-0 group-hover:opacity-100 p-1.5 text-slate-500 hover:text-rose-400 hover:bg-slate-800 rounded-lg transition-all"
          title="Eliminar hábito"
        >
          <Trash2 class="w-4 h-4" />
        </button>
      </div>
    </div>
  </div>
</template>
