<script setup lang="ts">
import { computed } from 'vue';
import { History, Calendar, Clock, MessageSquare } from 'lucide-vue-next';
import dayjs from 'dayjs';
import duration from 'dayjs/plugin/duration';
import Modal from '../common/Modal.vue';
import type { StreakReset } from '../../types/streak';

dayjs.extend(duration);

const props = defineProps<{
  isOpen: boolean;
  streakTitle: string;
  streakCreatedAt: string;
  resets: StreakReset[];
}>();

const emit = defineEmits<{
  (e: 'close'): void;
}>();

// Calculate interval duration for each reset
const intervals = computed(() => {
  // resets are sorted reset_at DESC
  return props.resets.map((reset, index) => {
    // Next item in descending order is previous chronological event
    const prevTimestamp = index < props.resets.length - 1
      ? props.resets[index + 1].reset_at
      : props.streakCreatedAt;

    const start = dayjs(prevTimestamp);
    const end = dayjs(reset.reset_at);
    const diffMs = Math.max(0, end.diff(start));
    const dur = dayjs.duration(diffMs);

    const days = Math.floor(dur.asDays());
    const hours = dur.hours();
    const minutes = dur.minutes();

    let durationText = '';
    if (days > 0) {
      durationText = `${days} día${days === 1 ? '' : 's'}${hours > 0 ? ` y ${hours}h` : ''}`;
    } else if (hours > 0) {
      durationText = `${hours} hora${hours === 1 ? '' : 's'}${minutes > 0 ? ` y ${minutes}m` : ''}`;
    } else {
      durationText = `${minutes} minuto${minutes === 1 ? '' : 's'}`;
    }

    return {
      id: reset.id,
      resetAt: reset.reset_at,
      formattedDate: dayjs(reset.reset_at).format('D MMMM YYYY, HH:mm'),
      reason: reset.reason,
      durationText,
      fromFormatted: dayjs(prevTimestamp).format('D MMM, HH:mm'),
    };
  });
});
</script>

<template>
  <Modal
    :is-open="isOpen"
    :title="`Historial de Reinicios: ${streakTitle}`"
    max-width="max-w-xl"
    @close="emit('close')"
  >
    <div v-if="intervals.length === 0" class="text-center py-8 text-slate-400">
      <div class="w-12 h-12 rounded-full bg-slate-800/80 flex items-center justify-center mx-auto mb-3 text-slate-500">
        <History class="w-6 h-6 text-sky-400" />
      </div>
      <p class="text-sm font-medium text-slate-300">¡Sin reinicios registrados!</p>
      <p class="text-xs text-slate-500 mt-1">
        Esta racha se ha mantenido intacta desde su creación el {{ dayjs(streakCreatedAt).format('D MMMM YYYY') }}.
      </p>
    </div>

    <div v-else class="space-y-3">
      <div class="text-xs text-slate-400 mb-2 flex items-center justify-between">
        <span>Total de reinicios: <strong class="text-slate-200">{{ intervals.length }}</strong></span>
        <span class="text-[11px] text-slate-500">Orden cronológico inverso</span>
      </div>

      <div
        v-for="(item, idx) in intervals"
        :key="item.id"
        class="p-3.5 rounded-lg bg-slate-950/70 border border-slate-800/80 hover:border-slate-700/80 transition-colors"
      >
        <div class="flex items-start justify-between gap-2">
          <div class="flex items-center gap-2">
            <span class="w-5 h-5 rounded-full bg-rose-500/10 text-rose-400 text-[10px] font-bold flex items-center justify-center border border-rose-500/30">
              #{{ intervals.length - idx }}
            </span>
            <div class="flex items-center gap-1.5 text-xs font-semibold text-slate-200">
              <Calendar class="w-3.5 h-3.5 text-slate-400" />
              <span>{{ item.formattedDate }}</span>
            </div>
          </div>

          <div class="flex items-center gap-1 px-2 py-0.5 rounded bg-amber-500/10 border border-amber-500/20 text-amber-400 text-[11px] font-mono font-medium">
            <Clock class="w-3 h-3" />
            <span>Duró: {{ item.durationText }}</span>
          </div>
        </div>

        <div v-if="item.reason" class="mt-2.5 pt-2 border-t border-slate-800/60 flex items-start gap-2 text-xs text-slate-300 italic bg-slate-900/40 p-2 rounded">
          <MessageSquare class="w-3.5 h-3.5 text-slate-500 shrink-0 mt-0.5" />
          <span>"{{ item.reason }}"</span>
        </div>
        <div v-else class="mt-2 text-[11px] text-slate-500 italic">
          Sin motivo especificado.
        </div>
      </div>
    </div>

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
