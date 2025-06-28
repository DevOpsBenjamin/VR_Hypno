<template>
  <div id="player-app" class="flex flex-col h-screen w-screen bg-brand-50">
    <div class="flex flex-1 overflow-hidden">
      <!-- Left Sidebar -->
      <div class="h-full" style="flex-basis: 20%; flex-shrink: 0; flex-grow: 0;">
        <PlayerMenu class="w-full h-full" />
      </div>
      <!-- Main Content -->
      <div class="h-full" style="flex-basis: 80%; flex-grow: 1; flex-shrink: 1;">
        <PlayerContent class="w-full h-full" />
      </div>
    </div>
    <!-- Bottom Player Bar -->
    <div style="height: 10vh; min-height: 60px; max-height: 20vh;" class="w-full">
      <PlayerBar class="w-full h-full" />
    </div>
  </div>
</template>

<script setup lang="ts">
import PlayerMenu from '@/components/PlayerMenu.vue';
import PlayerContent from '@/components/PlayerContent.vue';
import PlayerBar from '@/components/PlayerBar.vue';
import { goToEditor } from '@shared/utils/navigation';
import { usePlayerManager } from '@/composables/usePlayerManager';

const params = new URLSearchParams(window.location.search);
const uid = params.get('uid');
if (!uid) {
  alert('UID parameter is required');
  goToEditor(); // Redirect to player page if UID is missing
}
const playerManager = usePlayerManager();
// Initialize player manager
playerManager.initialize(uid);
</script>

<style scoped>
#player-app {
  min-height: 100vh;
}
</style>
