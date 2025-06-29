<template>
  <div class="flex items-center gap-2 h-8 bg-brand-100 rounded px-2 group hover:bg-brand-150 transition-colors">
    <div class="font-mono text-xs text-brand-600 w-12">{{ trackIndex }}</div>
    
    <!-- Track objects container -->
    <div class="flex-1 flex gap-1 min-h-6 bg-brand-50 rounded border border-brand-200 px-1 py-1">
      <TrackObjectBlock v-for="obj in track.objects" :key="obj.obj_id" :track-object="obj" />
      
      <!-- Empty state for track -->
      <div v-if="track.objects.length === 0" class="text-xs text-brand-400 flex items-center">
        {{ t('noObjectsOnTrack') }}
      </div>
    </div>
    
    <!-- Actions -->
    <div class="flex items-center gap-1">
      <button 
        @click="addObject"
        class="btn btn-xs bg-brand-500 hover:bg-brand-600 text-white px-1 py-1 rounded text-xs h-6"
        type="button"
      >
        + {{ t('addObject') }}
      </button>
      
      <button 
        @click="$emit('removeTrack')"
        class="btn btn-xs bg-brand-100 hover:bg-brand-300 text-red-500 hover:text-red-400 px-1 py-1 rounded text-xs opacity-0 group-hover:opacity-100 transition-opacity w-6 h-6 flex items-center justify-center"
        type="button"
        :title="t('removeTrack')"
      >
        {{ CrossEmoji }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import type { Track, TrackObj } from '@shared/domains/session/types';
import { CrossEmoji } from '@shared/icons/emoji';
import TrackObjectBlock from './TrackObjectBlock.vue';
import { t } from '@shared/utils/i18n';

defineProps<{ 
  track: Track;
  trackIndex: number;
}>();

const emit = defineEmits<{
  removeTrack: [];
}>();

function addObject(): void {
  // TODO: Implement object creation - for now just add a placeholder
  const newObject: TrackObj = {
    obj_id: `obj_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
    obj_type: 'placeholder',
    spawn: 0,
    despawn: 1000,
    timeline: []
  };
  
  // TODO: This should probably open a dialog to choose object type and properties
  console.log('TODO: Add object creation dialog', newObject);
}
</script>
