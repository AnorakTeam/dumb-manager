<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { Plus, Flame } from 'lucide-vue-next';
import { useStreaks } from '../composables/useStreaks';
import { useConfirm } from '../composables/useConfirm';
import { toast } from '../composables/useToast';
import StreakCard from '../components/streaks/StreakCard.vue';
import StreakCreateModal from '../components/streaks/StreakCreateModal.vue';
import StreakResetModal from '../components/streaks/StreakResetModal.vue';
import StreakHistoryDialog from '../components/streaks/StreakHistoryDialog.vue';
import type { StreakSummary, StreakReset } from '../types/streak';

const {
  streaks,
  loading,
  fetchStreaks,
  createStreak,
  resetStreak,
  getStreakHistory,
  deleteStreak,
} = useStreaks();

const { confirm } = useConfirm();

const isCreateModalOpen = ref(false);

const isResetModalOpen = ref(false);
const activeResetStreak = ref<StreakSummary | null>(null);

const isHistoryModalOpen = ref(false);
const activeHistoryStreak = ref<StreakSummary | null>(null);
const activeHistoryResets = ref<StreakReset[]>([]);

onMounted(() => {
  fetchStreaks();
});

async function handleCreate(title: string) {
  await createStreak(title);
  isCreateModalOpen.value = false;
  toast.success(`Racha "${title}" iniciada`);
}

function handleOpenReset(streakId: string) {
  const s = streaks.value.find((item) => item.id === streakId);
  if (s) {
    activeResetStreak.value = s;
    isResetModalOpen.value = true;
  }
}

async function handleConfirmReset(reason: string) {
  if (activeResetStreak.value) {
    const title = activeResetStreak.value.title;
    await resetStreak(activeResetStreak.value.id, reason);
    isResetModalOpen.value = false;
    activeResetStreak.value = null;
    toast.info(`Racha "${title}" reiniciada a 0 días`);
  }
}

async function handleOpenHistory(streakId: string) {
  const s = streaks.value.find((item) => item.id === streakId);
  if (s) {
    activeHistoryStreak.value = s;
    activeHistoryResets.value = await getStreakHistory(streakId);
    isHistoryModalOpen.value = true;
  }
}

async function handleDelete(streakId: string) {
  const streak = streaks.value.find((s) => s.id === streakId);
  const title = streak ? streak.title : 'esta racha';
  const ok = await confirm({
    title: '¿Eliminar racha?',
    message: `¿Estás seguro de que deseas eliminar la racha "${title}"? Se borrará todo su historial acumulado de reinicios.`,
    confirmText: 'Eliminar racha',
    cancelText: 'Cancelar',
    type: 'danger',
  });
  if (ok) {
    await deleteStreak(streakId);
    toast.success('Racha eliminada');
  }
}
</script>

<template>
  <div class="h-full w-full flex flex-col bg-slate-950 overflow-hidden">
    <!-- Top Header Bar -->
    <header class="p-6 border-b border-slate-800/80 bg-slate-900/40 flex items-center justify-between shrink-0">
      <div>
        <div class="flex items-center gap-2">
          <Flame class="w-5 h-5 text-amber-400" />
          <h2 class="text-lg font-bold text-white tracking-tight">Rachas (Days Since)</h2>
        </div>
        <p class="text-xs text-slate-400 mt-0.5">
          Monitorea los días y horas transcurridos desde que dejaste un mal hábito o iniciaste un nuevo ciclo.
        </p>
      </div>

      <button
        @click="isCreateModalOpen = true"
        class="flex items-center gap-2 px-3.5 py-2 bg-amber-500 hover:bg-amber-400 text-slate-950 font-bold text-xs rounded-lg shadow-sm shadow-amber-950 transition-colors cursor-pointer"
      >
        <Plus class="w-4 h-4" />
        <span>Nueva Racha</span>
      </button>
    </header>

    <!-- Main Content Area: Grid of Cards -->
    <main class="flex-1 overflow-y-auto p-6">
      <div v-if="streaks.length === 0 && !loading" class="h-full flex flex-col items-center justify-center text-center p-8 select-none">
        <div class="relative mb-4">
          <img
            src="/logo-avatar.png"
            alt="dumb-manager"
            class="w-60 h-60 rounded-2xl object-cover border border-slate-800 shadow-xl shadow-amber-500/10"
          />
        </div>
        <h3 class="text-base font-semibold text-slate-200">No hay ninguna racha creada</h3>
        <p class="text-xs text-slate-400 mt-1 max-w-sm">
          Crea contadores para seguir cuánto tiempo llevas sin comida rápida, o midiendo cualquier evento importante.
        </p>
        <button
          @click="isCreateModalOpen = true"
          class="mt-4 flex items-center gap-2 px-4 py-2 bg-amber-500 hover:bg-amber-400 text-slate-950 font-bold text-xs rounded-lg shadow-sm transition-colors cursor-pointer"
        >
          <Plus class="w-4 h-4" />
          <span>Crear Primer Contador</span>
        </button>
      </div>

      <div
        v-else
        class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5 max-w-7xl mx-auto"
      >
        <StreakCard
          v-for="s in streaks"
          :key="s.id"
          :streak="s"
          @reset="handleOpenReset"
          @history="handleOpenHistory"
          @delete="handleDelete"
        />
      </div>
    </main>

    <!-- Modals -->
    <StreakCreateModal
      :is-open="isCreateModalOpen"
      @close="isCreateModalOpen = false"
      @create="handleCreate"
    />

    <StreakResetModal
      v-if="activeResetStreak"
      :is-open="isResetModalOpen"
      :streak-title="activeResetStreak.title"
      @close="isResetModalOpen = false"
      @confirm="handleConfirmReset"
    />

    <StreakHistoryDialog
      v-if="activeHistoryStreak"
      :is-open="isHistoryModalOpen"
      :streak-title="activeHistoryStreak.title"
      :streak-created-at="activeHistoryStreak.created_at"
      :resets="activeHistoryResets"
      @close="isHistoryModalOpen = false"
    />
  </div>
</template>
