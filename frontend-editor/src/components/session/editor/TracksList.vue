<template>
  <div class="flex flex-col gap-2">
    <!-- Example: Render 3 tracks as placeholder -->
    <TrackRow v-for="track in 3" :key="track" :track-index="track" />
    <button class="mt-2 btn btn-primary w-fit" type="button">
      {{ t('sessionTracksAddTrack') }}
    </button>
  </div>
</template>

<script setup lang="ts">
import TrackRow from './TrackRow.vue';
import { t } from '@shared/utils/i18n';
</script>
