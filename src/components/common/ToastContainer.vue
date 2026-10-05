<script setup lang="ts">
import { useToast } from '../../composables/useToast';
import {
  CheckCircle2,
  AlertCircle,
  AlertTriangle,
  Info,
  X,
} from 'lucide-vue-next';

const { toasts, removeToast } = useToast();
</script>

<template>
  <Teleport to="body">
    <div
      class="fixed bottom-5 right-5 z-[9999] flex flex-col gap-2.5 pointer-events-none max-w-sm w-auto"
      aria-live="polite"
    >
      <TransitionGroup
        enter-active-class="transition duration-200 ease-out transform"
        enter-from-class="opacity-0 translate-x-8 scale-95"
        enter-to-class="opacity-100 translate-x-0 scale-100"
        leave-active-class="transition duration-150 ease-in transform absolute right-0"
        leave-from-class="opacity-100 translate-x-0 scale-100"
        leave-to-class="opacity-0 translate-x-8 scale-95"
        move-class="transition duration-200 ease-out"
      >
        <div
          v-for="t in toasts"
          :key="t.id"
          :class="[
            'pointer-events-auto flex items-center gap-3 px-3.5 py-2.5 rounded-xl border shadow-xl shadow-black/80 backdrop-blur-md select-none min-w-[280px] max-w-md',
            t.type === 'success'
              ? 'bg-slate-900/95 border-emerald-500/30 text-emerald-400'
              : t.type === 'error'
              ? 'bg-slate-900/95 border-rose-500/30 text-rose-400'
              : t.type === 'warning'
              ? 'bg-slate-900/95 border-amber-500/30 text-amber-400'
              : 'bg-slate-900/95 border-sky-500/30 text-sky-400',
          ]"
        >
          <!-- Icon -->
          <div class="shrink-0">
            <CheckCircle2 v-if="t.type === 'success'" class="w-4 h-4 text-emerald-400" />
            <AlertCircle v-else-if="t.type === 'error'" class="w-4 h-4 text-rose-400" />
            <AlertTriangle v-else-if="t.type === 'warning'" class="w-4 h-4 text-amber-400" />
            <Info v-else class="w-4 h-4 text-sky-400" />
          </div>

          <!-- Message -->
          <span class="flex-1 text-xs font-medium text-slate-100 leading-snug">
            {{ t.message }}
          </span>

          <!-- Close button -->
          <button
            type="button"
            @click="removeToast(t.id)"
            class="shrink-0 p-1 -mr-1 rounded-md text-slate-400 hover:text-slate-200 hover:bg-slate-800 transition-colors cursor-pointer"
            title="Cerrar"
          >
            <X class="w-3.5 h-3.5" />
          </button>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>
