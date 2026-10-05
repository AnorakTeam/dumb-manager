<script setup lang="ts">
import { FileText, Flame, CalendarCheck2, Database } from 'lucide-vue-next';

export type NavTab = 'notes' | 'streaks' | 'habits';

defineProps<{
  activeTab: NavTab;
}>();

const emit = defineEmits<{
  (e: 'update:activeTab', tab: NavTab): void;
}>();

const tabs: { id: NavTab; label: string; icon: any }[] = [
  { id: 'notes', label: 'Notas', icon: FileText },
  { id: 'streaks', label: 'Rachas (Days Since)', icon: Flame },
  { id: 'habits', label: 'Hábitos Diarios', icon: CalendarCheck2 },
];
</script>

<template>
  <aside
    class="w-64 h-full bg-slate-950 border-r border-slate-800/80 flex flex-col justify-between select-none shrink-0"
  >
    <!-- Brand / Header -->
    <div>
      <div class="px-5 py-4 border-b border-slate-800/60 flex flex-col items-center gap-3">
        <img
          src="/logo-avatar.png"
          alt="dumb-manager logo"
          class="w-22 h-22 rounded-xl object-cover bg-slate-900 border border-slate-700/80 shadow-md shadow-sky-500/10"
        />
        <div>
          <h1 class="text-2xl font-bold tracking-tight text-white leading-none">dumb-manager</h1>
          <!-- <span class="text-[11px] text-slate-400 font-mono tracking-wider">LOCAL FIRST</span> -->
        </div>
      </div>

      <!-- Navigation tabs -->
      <nav class="p-3 space-y-1.5 mt-2">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          @click="emit('update:activeTab', tab.id)"
          :class="[
            'w-full flex items-center gap-3 px-3.5 py-2.5 rounded-lg text-xs font-medium transition-all text-left group',
            activeTab === tab.id
              ? 'bg-sky-500/10 text-sky-400 border border-sky-500/30 shadow-sm shadow-sky-950'
              : 'text-slate-400 hover:text-slate-200 hover:bg-slate-900 border border-transparent',
          ]"
        >
          <component
            :is="tab.icon"
            :class="[
              'w-4 h-4 transition-transform group-hover:scale-110',
              activeTab === tab.id ? 'text-sky-400' : 'text-slate-400 group-hover:text-slate-300',
            ]"
          />
          <span>{{ tab.label }}</span>
        </button>
      </nav>
    </div>

    <!-- Footer System Status -->
    <div class="p-3 border-t border-slate-800/60">
      <div class="px-3 py-2 rounded-md bg-slate-900/60 border border-slate-800/50 flex items-center justify-between text-[11px] text-slate-400">
        <div class="flex items-center gap-2">
          <Database class="w-3.5 h-3.5 text-emerald-400" />
          <span>SQLite Embebido</span>
        </div>
        <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
      </div>
    </div>
  </aside>
</template>
