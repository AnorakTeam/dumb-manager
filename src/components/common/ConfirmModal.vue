<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch, nextTick } from 'vue';
import { AlertTriangle, AlertCircle, Info, X } from 'lucide-vue-next';
import { useConfirm } from '../../composables/useConfirm';

const { isOpen, options, handleConfirm, handleCancel } = useConfirm();

const cancelButtonRef = ref<HTMLButtonElement | null>(null);

function handleKeyDown(e: KeyboardEvent) {
  if (!isOpen.value) return;

  if (e.key === 'Escape') {
    e.preventDefault();
    handleCancel();
  }
}

watch(isOpen, (val) => {
  if (val) {
    nextTick(() => {
      cancelButtonRef.value?.focus();
    });
  }
});

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
});
</script>

<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition duration-150 ease-out"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="transition duration-100 ease-in"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        v-if="isOpen"
        class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/70 backdrop-blur-sm"
        @click.self="handleCancel"
      >
        <div
          class="w-full max-w-md rounded-xl bg-slate-900 border border-slate-800 shadow-2xl shadow-black/80 overflow-hidden transform transition-all select-none animate-in fade-in zoom-in-95 duration-150"
        >
          <!-- Header & Content -->
          <div class="p-5">
            <div class="flex items-start gap-3.5">
              <!-- Type Icon -->
              <div
                :class="[
                  'p-2.5 rounded-xl shrink-0 border',
                  options.type === 'danger'
                    ? 'bg-rose-500/10 text-rose-400 border-rose-500/20'
                    : options.type === 'warning'
                    ? 'bg-amber-500/10 text-amber-400 border-amber-500/20'
                    : 'bg-sky-500/10 text-sky-400 border-sky-500/20',
                ]"
              >
                <AlertTriangle v-if="options.type === 'danger'" class="w-5 h-5" />
                <AlertCircle v-else-if="options.type === 'warning'" class="w-5 h-5" />
                <Info v-else class="w-5 h-5" />
              </div>

              <div class="flex-1 min-w-0">
                <div class="flex items-start justify-between gap-2">
                  <h3 class="text-sm font-bold text-slate-100 leading-tight">
                    {{ options.title }}
                  </h3>
                  <button
                    @click="handleCancel"
                    class="p-1 -mr-1 -mt-1 text-slate-400 hover:text-slate-200 hover:bg-slate-800 rounded-lg transition-colors cursor-pointer"
                    title="Cerrar (Esc)"
                  >
                    <X class="w-4 h-4" />
                  </button>
                </div>
                <p class="mt-2 text-xs text-slate-300 leading-relaxed selectable">
                  {{ options.message }}
                </p>
              </div>
            </div>
          </div>

          <!-- Footer Actions -->
          <div class="px-5 py-3.5 border-t border-slate-800/80 bg-slate-950/50 flex justify-end gap-2.5">
            <button
              ref="cancelButtonRef"
              type="button"
              @click="handleCancel"
              class="px-3.5 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 active:bg-slate-800 text-slate-200 text-xs font-medium transition-colors cursor-pointer"
            >
              {{ options.cancelText || 'Cancelar' }}
            </button>
            <button
              type="button"
              @click="handleConfirm"
              :class="[
                'px-4 py-1.5 rounded-lg text-xs font-semibold shadow-sm transition-all cursor-pointer',
                options.type === 'danger'
                  ? 'bg-rose-600 hover:bg-rose-500 active:bg-rose-700 text-white shadow-rose-950/50'
                  : 'bg-sky-600 hover:bg-sky-500 active:bg-sky-700 text-white shadow-sky-950/50',
              ]"
            >
              {{ options.confirmText || 'Confirmar' }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>
