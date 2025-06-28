<template>
  <div class="flex items-center justify-between mb-2">
    <div class="font-bold text-lg">{{ t('sessionHeaderTitle') }}</div>
    <div class="text-sm text-brand-400">{{ t('sessionHeaderSongInfo') }}</div>
  </div>
</template>

<script setup lang="ts">
import { t } from '@shared/utils/i18n';
</script>
