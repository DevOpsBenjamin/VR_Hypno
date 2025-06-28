<template>
  <div class="flex flex-col gap-4 h-full">
    <!-- Header: Session/Song Info -->
    <SessionHeader />

    <div class="flex flex-1 gap-4">
      <!-- Timeline & Tracks -->
      <div class="flex-1 flex flex-col bg-white rounded-xl shadow p-4 overflow-hidden">
        <TimelineHeader />
        <TracksList />
      </div>

      <!-- Preview Canvas -->
      <div class="w-1/3 min-w-[320px] flex flex-col bg-white rounded-xl shadow p-4">
        <PreviewCanvas />
      </div>
    </div>

    <!-- Object Edit Popup (modal, hidden by default) -->
    <ObjectEditModal />
  </div>
</template>

<script setup lang="ts">
import SessionHeader from './editor/SessionHeader.vue';
import TimelineHeader from './editor/TimelineHeader.vue';
import TracksList from './editor/TracksList.vue';
import PreviewCanvas from './editor/PreviewCanvas.vue';
import ObjectEditModal from './editor/ObjectEditModal.vue';
// TODO: Add props, emits, and composables as needed
</script>

<style scoped>
/* Add any extra layout tweaks if needed */
</style>