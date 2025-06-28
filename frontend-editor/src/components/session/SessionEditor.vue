<template>
  <div class="flex flex-col h-full gap-1">
    <!-- Timeline & Tracks (Top, full width, 35% height) -->
    <div class="flex flex-col bg-white rounded-xl shadow p-1 overflow-hidden" style="flex: 0 0 35%; min-height: 120px; max-height: 35%;">
      <TimelineHeader />
      <div class="flex-1 overflow-y-auto">
        <TracksList />
      </div>
    </div>

    <!-- Main Content: Preview (65%) + Menu (35%) -->
    <div class="flex flex-1 min-h-0">
      <!-- Preview Canvas (65% width) -->
      <div class="flex-[0_0_65%] flex flex-col bg-white rounded-xl shadow p-1 min-h-0">
        <PreviewCanvas />
      </div>
      <!-- Menu (35% width) -->
      <div class="flex-[0_0_35%] flex flex-col bg-white rounded-xl shadow p-1 min-h-0">
        <SessionMenu />
      </div>
    </div>

    <!-- Object Edit Popup (modal, hidden by default) -->
    <ObjectEditModal />
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, inject } from 'vue';
import TimelineHeader from './editor/TimelineHeader.vue';
import TracksList from './editor/TracksList.vue';
import PreviewCanvas from './editor/PreviewCanvas.vue';
import SessionMenu from './editor/SessionMenu.vue';
import ObjectEditModal from './editor/ObjectEditModal.vue';
import { useNavigationStore } from '@/store/navigation';
import { EditorManager } from '@/app/EditorManager';

//INIT
const navStore = useNavigationStore();
const editor = reactive(inject<EditorManager>('editor')!)

onMounted(async () => {
  await editor.loadSession(navStore.options.uid as string);
});
</script>

<style scoped>
.flex-1.overflow-y-auto {
  min-height: 0;
}
</style>