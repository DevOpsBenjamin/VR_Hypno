<template>
  <div class="flex items-center gap-2 h-12 bg-brand-100 rounded px-2">
    <div class="font-mono text-xs text-brand-600">Track {{ trackIndex }}</div>
    <!-- TODO: Render objects on this track as blocks -->
    <div class="flex-1 flex gap-1">
      <TrackObjectBlock v-for="obj in 2" :key="obj" />
    </div>
    <button class="btn btn-xs btn-secondary" type="button">
      {{ t('sessionTracksAddObject') }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue';
import TrackObjectBlock from './TrackObjectBlock.vue';
import { t } from '@shared/utils/i18n';

defineProps<{ trackIndex: number }>();
</script>
