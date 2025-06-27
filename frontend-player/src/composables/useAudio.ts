import { ref, onMounted, onUnmounted, Ref } from 'vue';
import { Howl } from 'howler';
import type { Song } from '@shared/types';

export function useAudio(currentSong: Ref<Song | null>) {
  const isPlaying = ref(false);
  const howl = ref<Howl | null>(null);

  function play() {
    if (howl.value) {
      howl.value.play();
      isPlaying.value = true;
    }
  }

  function pause() {
    if (howl.value) {
      howl.value.pause();
      isPlaying.value = false;
    }
  }

  function stop() {
    if (howl.value) {
      howl.value.stop();
      isPlaying.value = false;
    }
  }

  onMounted(() => {
    if (currentSong.value && currentSong.value.info.audioUrl) {
      howl.value = new Howl({ src: [currentSong.value.info.audioUrl] });
    }
  });

  onUnmounted(() => {
    if (howl.value) {
      howl.value.unload();
      howl.value = null;
    }
  });

  return { isPlaying, play, pause, stop };
}
