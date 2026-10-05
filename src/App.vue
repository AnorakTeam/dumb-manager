<script setup lang="ts">
import { ref } from 'vue';
import Sidebar, { type NavTab } from './components/common/Sidebar.vue';
import NotesView from './views/NotesView.vue';
import StreaksView from './views/StreaksView.vue';
import HabitsView from './views/HabitsView.vue';
import ConfirmModal from './components/common/ConfirmModal.vue';
import ToastContainer from './components/common/ToastContainer.vue';

const activeTab = ref<NavTab>('notes');
</script>

<template>
  <div class="flex h-screen w-screen bg-slate-950 text-slate-100 overflow-hidden font-sans select-none antialiased">
    <!-- Sidebar -->
    <Sidebar v-model:active-tab="activeTab" />

    <!-- Main View Workspace -->
    <div class="flex-1 h-full min-w-0 overflow-hidden relative">
      <Transition
        enter-active-class="transition-opacity duration-150 ease-out"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition-opacity duration-100 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
        mode="out-in"
      >
        <KeepAlive>
          <component
            :is="
              activeTab === 'notes'
                ? NotesView
                : activeTab === 'streaks'
                ? StreaksView
                : HabitsView
            "
          />
        </KeepAlive>
      </Transition>
    </div>

    <!-- Global In-App Dialogs and Notifications -->
    <ConfirmModal />
    <ToastContainer />
  </div>
</template>
