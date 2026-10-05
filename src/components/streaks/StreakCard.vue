<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { Flame, RotateCcw, History, Trash2, Clock, AlertCircle } from 'lucide-vue-next';
import dayjs from 'dayjs';
import duration from 'dayjs/plugin/duration';
import type { StreakSummary } from '../../types/streak';

dayjs.extend(duration);

const props = defineProps<{
  streak: StreakSummary;
}>();

const emit = defineEmits<{
  (e: 'reset', streakId: string): void;
  (e: 'history', streakId: string): void;
  (e: 'delete', streakId: string): void;
}>();

const now = ref(dayjs());
let intervalTimer: any = null;

onMounted(() => {
  intervalTimer = setInterval(() => {
    now.value = dayjs();
  }, 10000); // update every 10s for accurate live count
});

onUnmounted(() => {
  if (intervalTimer) clearInterval(intervalTimer);
});

const elapsed = computed(() => {
  const start = dayjs(props.streak.effective_since);
  const diffMs = Math.max(0, now.value.diff(start));
  const dur = dayjs.duration(diffMs);

  const days = Math.floor(dur.asDays());
  const hours = dur.hours();
  const minutes = dur.minutes();

  return {
    days,
    hours,
    minutes,
    totalMinutes: Math.floor(dur.asMinutes()),
  };
});

function formatDate(dateStr: string) {
  try {
    return dayjs(dateStr).format('D MMM YYYY, HH:mm');
  } catch {
    return dateStr;
  }
}
</script>

<template>
  <div
    class="p-5 rounded-xl bg-slate-900/80 border border-slate-800 hover:border-slate-700/80 shadow-lg shadow-black/40 flex flex-col justify-between transition-all group relative overflow-hidden"
  >
    <!-- Background subtle flame glow -->
    <div
      class="absolute -right-8 -top-8 w-32 h-32 bg-amber-500/5 rounded-full blur-2xl pointer-events-none group-hover:bg-amber-500/10 transition-colors"
    ></div>

    <div>
      <!-- Header: Title and Delete -->
      <div class="flex items-start justify-between gap-3">
        <div class="flex items-center gap-2">
          <div class="p-2 rounded-lg bg-amber-500/10 text-amber-400 border border-amber-500/20">
            <Flame class="w-4 h-4" />
          </div>
          <div>
            <h3 class="text-sm font-bold text-slate-100 group-hover:text-white transition-colors">
              {{ streak.title }}
            </h3>
            <span class="text-[11px] text-slate-400 font-mono">
              {{ streak.last_reset_at ? 'Último reinicio' : 'Iniciado' }}: {{ formatDate(streak.effective_since) }}
            </span>
          </div>
        </div>

        <button
          @click="emit('delete', streak.id)"
          class="opacity-0 group-hover:opacity-100 p-1.5 text-slate-500 hover:text-rose-400 hover:bg-slate-800 rounded-lg transition-all"
          title="Eliminar racha"
        >
          <Trash2 class="w-4 h-4" />
        </button>
      </div>

      <!-- Giant Counter -->
      <div class="my-5 text-center py-2 px-3 rounded-lg bg-slate-950/60 border border-slate-800/60">
        <template v-if="elapsed.days > 0">
          <div class="text-4xl font-extrabold tracking-tight text-white font-mono flex items-baseline justify-center gap-1.5">
            <span class="text-amber-400 text-5xl">{{ elapsed.days }}</span>
            <span class="text-xs font-semibold text-slate-400 tracking-wider">
              {{ elapsed.days === 1 ? 'DÍA' : 'DÍAS' }}
            </span>
          </div>
          <p class="text-[11px] text-slate-400 font-mono mt-1">
            + {{ elapsed.hours }}h {{ elapsed.minutes }}m transcurridos
          </p>
        </template>
        <template v-else-if="elapsed.hours > 0">
          <div class="text-4xl font-extrabold tracking-tight text-white font-mono flex items-baseline justify-center gap-1.5">
            <span class="text-amber-400 text-4xl">{{ elapsed.hours }}</span>
            <span class="text-xs font-semibold text-slate-400 tracking-wider">
              {{ elapsed.hours === 1 ? 'HORA' : 'HORAS' }}
            </span>
          </div>
          <p class="text-[11px] text-slate-400 font-mono mt-1">
            + {{ elapsed.minutes }}m transcurridos
          </p>
        </template>
        <template v-else>
          <div class="text-4xl font-extrabold tracking-tight text-white font-mono flex items-baseline justify-center gap-1.5">
            <span class="text-amber-400 text-4xl">{{ elapsed.minutes }}</span>
            <span class="text-xs font-semibold text-slate-400 tracking-wider">MINUTOS</span>
          </div>
          <p class="text-[11px] text-emerald-400 font-mono mt-1 flex items-center justify-center gap-1">
            <Clock class="w-3 h-3" />
            <span>¡Racha recién iniciada!</span>
          </p>
        </template>
      </div>

      <!-- Last reason if present -->
      <div
        v-if="streak.last_reason"
        class="mb-4 px-2.5 py-1.5 rounded bg-slate-950/40 border border-slate-800/40 text-[11px] text-slate-400 flex items-start gap-1.5 italic"
      >
        <AlertCircle class="w-3.5 h-3.5 text-slate-500 shrink-0 mt-0.5" />
        <span class="line-clamp-1">"{{ streak.last_reason }}"</span>
      </div>
    </div>

    <!-- Actions & Stats Footer -->
    <div class="pt-3 border-t border-slate-800/60 flex items-center justify-between gap-2">
      <button
        @click="emit('history', streak.id)"
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-slate-800/60 hover:bg-slate-800 text-slate-300 hover:text-white text-xs font-medium border border-slate-700/50 transition-colors"
      >
        <History class="w-3.5 h-3.5 text-slate-400" />
        <span>Historial ({{ streak.total_resets }})</span>
      </button>

      <button
        @click="emit('reset', streak.id)"
        class="flex items-center gap-1.5 px-3.5 py-1.5 rounded-lg bg-rose-500/10 hover:bg-rose-500 text-rose-400 hover:text-white text-xs font-semibold border border-rose-500/30 transition-all shadow-sm"
      >
        <RotateCcw class="w-3.5 h-3.5" />
        <span>Reiniciar</span>
      </button>
    </div>
  </div>
</template>
