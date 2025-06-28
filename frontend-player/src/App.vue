<template>
  <div id="player-app">
    <h1>{{ t('appTitle') }}</h1>
    <!-- CPU Tracking
    <div v-if="playlist">
      <h2>{{ playlist.info.name }}</h2>
      <div v-if="currentSong">
        <h3>{{ currentSong.info.name }}</h3>
        <audio ref="audio" :src="currentSong.info.audioUrl" controls @play="onPlay" @pause="onPause" @ended="onEnded"></audio>
      </div>
      <div v-else>
        <p>{{ t('noSongs').value }}</p>
      </div>
    </div>
    <div v-else>
      <p>{{ t('loadingPlaylist').value }}</p>
    </div>
    <button id="to-editor" @click="goToEditor">{{ t('editor').value }}</button>
    <button v-if="xrSupported" @click="xrActive ? stopXR() : startXR()">
      {{ xrActive ? 'Stop VR' : 'Start VR' }}
    </button>
    <canvas id="xr-canvas" class="w-full" style="height:400px;display:block;margin:2rem auto 0;"></canvas>
    
    <div v-if="cpuUsage !== null">
      <small>CPU usage (approx): {{ cpuUsage }}%</small>
    </div> -->
  </div>
</template>

<script setup lang="ts">
import { t } from '@shared/utils/i18n';
/*
import { ref } from 'vue';
import { usePlaylist } from './composables/usePlaylist';
import { useCPUTracker } from './composables/useCPUTracker';
import { useXR } from './composables/useXR';
import { useAudio } from './composables/useAudio';
import { useThreeJS } from './composables/useThreeJS';

const { playlist, currentSession, currentSong, currentThreeJSConfig, loading, error } = usePlaylist('demo');
const { cpuUsage } = useCPUTracker();
const { isSupported: xrSupported, isActive: xrActive, startXR, stopXR } = useXR('xr-canvas');
const { isPlaying, play, pause, stop } = useAudio(currentSong);
const { renderer, scene, camera } = useThreeJS('xr-canvas', currentThreeJSConfig);
*/

function onPlay() {
  //play();
}
function onPause() {
  //pause();
}
function onEnded() {
  //stop();
}
</script>

<style scoped>
#player-app {
  max-width: 600px;
  margin: 2rem auto;
  padding: 2rem;
  background: #222;
  color: #fff;
  border-radius: 1rem;
  box-shadow: 0 0 20px #0008;
}
button {
  margin-top: 1rem;
  padding: 0.5rem 1rem;
  background: #4caf50;
  color: #fff;
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
}
button:hover {
  background: #388e3c;
}
</style>
