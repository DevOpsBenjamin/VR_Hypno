<template>
  <div class="flex items-center gap-1 h-7 bg-brand-100 rounded px-1 group hover:bg-brand-150 transition-colors">
    <div class="font-mono text-xs text-brand-600 w-3 text-center flex-shrink-0">{{ trackIndex }}</div>
    
    <!-- Track objects container -->
    <div class="flex-1 flex gap-1 min-h-5 bg-brand-50 rounded border border-brand-200 px-1 py-0.5 overflow-x-auto">
      <TrackObjectBlock v-for="obj in track?.objects || []" :key="obj.obj_id" :track-object="obj" />
      
      <!-- Empty state for track -->
      <div v-if="!track?.objects || track.objects.length === 0" class="text-xs text-brand-400 flex items-center truncate">
        {{ t('noObjectsOnTrack') }}
      </div>
    </div>
    
    <!-- Actions -->
    <div class="flex items-center gap-1">
      <button 
        @click="addObject"
        class="btn btn-xs bg-brand-500 hover:bg-brand-600 text-white px-1 py-0.5 rounded text-xs h-5"
        type="button"
      >
        + {{ t('addObject') }}
      </button>
      
      <button 
        @click="removeTrack"
        class="btn btn-xs bg-brand-100 hover:bg-brand-300 text-red-500 hover:text-red-400 px-1 py-0.5 rounded text-xs opacity-0 group-hover:opacity-100 transition-opacity w-5 h-5 flex items-center justify-center"
        type="button"
        :title="t('removeTrack')"
      >
        {{ CrossEmoji }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, inject, reactive } from 'vue';
import { EditorManager } from '@/app/EditorManager';
import type { Track, TrackObj } from '@shared/domains/session/types';
import { t } from '@shared/utils/i18n';
import { defineProps, defineEmits } from 'vue';
import { CrossEmoji } from '@shared/icons/emoji';
import TrackObjectBlock from './TrackObjectBlock.vue';

const editor = reactive(inject<EditorManager>('editor')!);

const props = defineProps<{ 
  trackId: string;
}>();

// Find the track from the editor using trackId
const track = computed(() => 
  editor.currentSession?.info.tracks.find(t => t.id === props.trackId)
);

// Compute the display index dynamically
const trackIndex = computed(() => {
  if (!editor.currentSession) return 0;
  return editor.currentSession.info.tracks.findIndex(t => t.id === props.trackId) + 1;
});

function removeTrack(): void {
  if (!editor.currentSession) return;
  
  const trackIndex = editor.currentSession.info.tracks.findIndex(t => t.id === props.trackId);
  if (trackIndex !== -1) {
    editor.currentSession.info.tracks.splice(trackIndex, 1);
  }
}

function addObject(): void {
  if (!track.value) return;
  
  const newObject: TrackObj = {
    obj_id: `obj_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
    obj_type: 'placeholder',
    spawn: 0,
    despawn: 1000,
    timeline: []
  };
  
  track.value.objects.push(newObject);
  
  // TODO: This should probably open a dialog to choose object type and properties
  console.log('TODO: Add object creation dialog', newObject);
}
</script>
