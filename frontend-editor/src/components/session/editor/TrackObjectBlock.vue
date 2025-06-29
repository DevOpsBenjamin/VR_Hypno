<template>
  <div 
    class="h-4 bg-brand-300 hover:bg-brand-400 rounded shadow flex items-center justify-center cursor-pointer transition-colors px-1 min-w-8"
    :title="`${trackObject.obj_type} (${trackObject.spawn}ms - ${trackObject.despawn}ms)`"
  >
    <span class="text-xs text-white font-medium truncate">
      {{ trackObject.obj_type }}
    </span>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue';
import type { TrackObj } from '@shared/domains/session/types';

defineProps<{
  trackObject: TrackObj;
}>();
</script>
