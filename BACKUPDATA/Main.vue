<template>
  <h1>Vite HOTRELOAD</h1>
  <h1>VR Hypno - Mode Éditeur</h1>
  <button @click="goToPlayer">Basculer en mode Player</button>
</template>

<script setup lang="ts">
import { useI18n, Locale } from '../shared/utils/i18n';
import { ref } from 'vue';

const isDev = import.meta.env && import.meta.env.DEV;

const locale = ref<Locale>('fr'); // or 'en', can be dynamic
const t = useI18n(locale);

// Usage: t('appTitle').value

async function goToPlayer() {
  if (isDev) {
    window.location.replace("http://localhost:5174");
  } else {
    window.location.replace("/player/index.html");
  }
}
</script>
