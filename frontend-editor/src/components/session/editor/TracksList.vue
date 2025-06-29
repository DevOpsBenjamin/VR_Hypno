<template>
  <div class="flex flex-col gap-1 mt-4">
    <!-- Tracks header with add button -->
    <div class="flex items-center justify-between bg-brand-50 rounded px-2 py-1 border text-sm">
      <h3 class="font-medium text-brand-700 text-xs">{{ t('tracks') }}</h3>
      <button 
        @click="addTrack"
        class="btn btn-xs bg-brand-500 hover:bg-brand-600 text-white rounded px-2 py-1 flex items-center gap-1 transition text-xs"
        type="button"
      >
        <svg class="w-3 h-3" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
          <path d="M12 5v14M5 12h14"/>
        </svg>
        {{ t('addTrack') }}
      </button>
    </div>

    <!-- Render actual tracks -->
    <TrackRow 
      v-for="(track, index) in tracks" 
      :key="track.id" 
      :track="track"
      :track-index="index + 1" 
      @remove-track="removeTrack(index)"
    />

    <!-- Empty state when no tracks -->
    <div v-if="tracks.length === 0" class="text-center py-8 text-brand-400 bg-brand-50 rounded-lg border-2 border-dashed border-brand-200">
      <svg class="w-12 h-12 mx-auto mb-2 text-brand-300" fill="none" stroke="currentColor" stroke-width="1" viewBox="0 0 24 24">
        <path d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM22 16c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2z"/>
      </svg>
      <p class="text-sm">No tracks yet</p>
      <p class="text-xs mt-1">Click "Add Track" to start</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, inject } from 'vue';
import { EditorManager } from '@/app/EditorManager';
import type { Track } from '@shared/domains/session/types';

import TrackRow from './TrackRow.vue';
import { t } from '@shared/utils/i18n';

const editor = inject<EditorManager>('editor')!;

// Reactive reference to tracks
const tracks = computed(() => editor.currentSession?.info.tracks || []);

function addTrack(): void {
  if (!editor.currentSession) return;
  
  const newTrack: Track = {
    id: `track_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
    objects: []
  };
  
  editor.currentSession.info.tracks.push(newTrack);
}

function removeTrack(index: number): void {
  if (!editor.currentSession) return;
  
  editor.currentSession.info.tracks.splice(index, 1);
}
</script>
