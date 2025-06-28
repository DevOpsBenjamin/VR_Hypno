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
    <div class="flex flex-1 gap-1 min-h-0">
      <!-- Preview Canvas (65% width) -->
      <div class="flex-[0_0_65%] flex flex-col bg-white rounded-xl shadow p-1 min-h-0">
        <PreviewCanvas />
      </div>
      <!-- Menu (35% width) -->
      <div class="flex-[0_0_35%] flex flex-col bg-white rounded-xl shadow p-4 min-h-0">
        <SessionMenu
          :session="session"
          :loading="loading"
          :saving="saving"
          @save="onSave"
        />
      </div>
    </div>

    <!-- Object Edit Popup (modal, hidden by default) -->
    <ObjectEditModal />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { t } from '@shared/utils/i18n';
import TimelineHeader from './editor/TimelineHeader.vue';
import TracksList from './editor/TracksList.vue';
import PreviewCanvas from './editor/PreviewCanvas.vue';
import SessionMenu from './editor/SessionMenu.vue';
import ObjectEditModal from './editor/ObjectEditModal.vue';

import { useNavigationStore } from '@/store/navigation';
import { useSessionEditor } from '@/composables/useSessionEditor';

const navStore = useNavigationStore()
const uid = computed(() => navStore.options.uid as string)

const {
  loading, error, session, saving, load, save
} = useSessionEditor(uid.value);

function onSave() {
  save();
}
</script>

<style scoped>
.flex-1.overflow-y-auto {
  min-height: 0;
}
</style>