<template>
  <div class="flex flex-col h-full gap-4">
    <!-- Timeline & Tracks (Top, full width, 1/4 height) -->
    <div class="flex flex-col bg-white rounded-xl shadow p-4 overflow-hidden" style="flex: 0 0 25%; min-height: 120px; max-height: 25%;">
      <TimelineHeader />
      <div class="flex-1 overflow-y-auto">
        <TracksList />
      </div>
    </div>

    <!-- Main Content: Preview + Properties (Bottom, 3/4 height) -->
    <div class="flex flex-1 gap-4 min-h-0">
      <!-- Preview Canvas (takes most of the space) -->
      <div class="flex-1 flex flex-col bg-white rounded-xl shadow p-4 min-h-0">
        <PreviewCanvas />
      </div>
      <!-- Properties Panel (optional, right side) -->
      <!-- <div class="w-80 min-w-[240px] flex flex-col bg-white rounded-xl shadow p-4">
        <ObjectPropertiesPanel />
      </div> -->
    </div>

    <!-- Object Edit Popup (modal, hidden by default) -->
    <ObjectEditModal />
  </div>
</template>

<script setup lang="ts">
import TimelineHeader from './editor/TimelineHeader.vue';
import TracksList from './editor/TracksList.vue';
import PreviewCanvas from './editor/PreviewCanvas.vue';
import ObjectEditModal from './editor/ObjectEditModal.vue';
// TODO: Add props, emits, and composables as needed
</script>

<style scoped>
/* Ensure the timeline/tracks area stays at the top and is scrollable if too many tracks */
.flex-1.overflow-y-auto {
  min-height: 0;
}
</style>