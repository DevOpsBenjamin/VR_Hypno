<template>
  <div class="flex items-center gap-2 mb-2">
    <div class="text-xs text-brand-400">{{ t('sessionTimelineTime') }}</div>
    <!-- TODO: Add zoom controls, time ruler, playhead, etc. -->
  </div>
</template>

<script setup lang="ts">
import { t } from '@shared/utils/i18n';
</script>
