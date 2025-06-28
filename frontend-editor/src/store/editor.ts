import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { updateSession, getSession } from '@shared/domains/session/endpoints';
import { getSong } from '@shared/domains/song/endpoints';
import { Session } from '@shared/domains/session/types';
import { Song } from '@shared/domains/song/types';

export const useEditorStore = defineStore('editor', () => {
  // State
  const session = ref<Session | null>(null);
  const song = ref<Song | null>(null);
  const selectedObjectId = ref<string | null>(null);
  const isPlaying = ref(false);
  const currentTime = ref(0);
  const isObjectModalOpen = ref(false);
  
  // Loading states
  const loading = ref({
    session: false,
    song: false
  });
  
  // Error states
  const errors = ref<string | null>(null);

  // Getters
  const isLoading = computed(() => 
    loading.value.session || loading.value.song
  );  
  const hasErrors = computed(() => !!errors.value);  
  const duration = computed(() => 
    song.value?.info.duration || 0
  );
  // Editor actions
  function selectObject(objectId: string | null) {
    selectedObjectId.value = objectId;
  }

  function openObjectModal(objectId?: string) {
    if (objectId) {
      selectObject(objectId);
    }
    isObjectModalOpen.value = true;
  }
  function closeObjectModal() {
    isObjectModalOpen.value = false;
  }

  // Playback controls
  function play() {
    isPlaying.value = true;
  }
  function pause() {
    isPlaying.value = false;
  }
  function seek(time: number) {
    currentTime.value = Math.max(0, Math.min(time, duration.value));
  }

  // Reset store
  function reset() {
    session.value = null;
    song.value = null;
    selectedObjectId.value = null;
    isPlaying.value = false;
    currentTime.value = 0;
    isObjectModalOpen.value = false;
    
    loading.value = {
      session: false,
      song: false,
    };
    
    errors.value = null;
  }

  return {
    // State
    session,
    song,
    selectedObjectId,
    isPlaying,
    currentTime,
    isObjectModalOpen,
    loading,
    errors,
    
    // Getters
    isLoading,
    hasErrors,
    duration,
    
    // Actions
    saveSession,
    selectObject,
    openObjectModal,
    closeObjectModal,
    play,
    pause,
    seek,
    reset
  };
});