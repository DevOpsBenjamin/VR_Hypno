<template>
  <h1>Vite HOTRELOAD</h1>
  <h1>VR Hypno - Mode Éditeur</h1>
  <button @click="goToPlayer">Basculer en mode Player</button>
</template>

<script setup lang="ts">
const isDev = import.meta.env && import.meta.env.DEV;

async function goToPlayer() {
  if (isDev) {
    window.location.replace("http://localhost:5174");
  } else {
    window.location.replace("/player/index.html");
  }
}
</script>
