<template>
  <aside class="flex flex-col h-full w-full bg-brand-100 p-4 border-r border-brand-200">
    <!-- Lang Switcher -->
    <div class="flex items-center justify-center gap-2 mb-6">
      <button @click="setLocale('en')" :class="[localeStore.lang === 'en' ? 'ring-2 ring-brand-400' : '', 'rounded-full p-1 transition']" title="English">
        <img src="/flag-gb.svg" alt="GB" class="w-10 h-6 rounded shadow-sm" />
      </button>
      <button @click="setLocale('fr')" :class="[localeStore.lang === 'fr' ? 'ring-2 ring-brand-400' : '', 'rounded-full p-1 transition']" title="Français">
        <img src="/flag-fr.svg" alt="FR" class="w-10 h-6 rounded shadow-sm" />
      </button>
    </div>
    <!-- Back to Editor Button -->
    <button @click="goToEditor" class="mb-6 w-full bg-brand-500 hover:bg-brand-600 text-white font-bold py-2 rounded-lg shadow transition border border-brand-300">
      {{ t('editor') }}
    </button>
    <!-- Sessions List -->
    <div class="flex-1 flex flex-col gap-3 overflow-y-auto">
      <SessionCard v-for="session in sessions" :key="session.uid" :session="session" />
    </div>
  </aside>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useLocaleStore } from '@/store/locale';
import { t } from '@shared/utils/i18n';
import { goToEditor } from '@shared/utils/navigation';
import SessionCard from '@/components/SessionCard.vue';
import { usePlayerManager } from '@/composables/usePlayerManager';
import { Session } from '@shared/domains/session/types';

const sessions = ref<Session[]>([]);
const playerManager = usePlayerManager();

watch(
  () => playerManager.isInitialized.value,
  (val) => {
    console.log('watch isInitialized:', val);
    sessions.value = [...playerManager.sessions];
  },
  { immediate: true }
);

const localeStore = useLocaleStore();
function setLocale(lang: 'en' | 'fr') {
  localeStore.setLocale(lang);
}
</script>

<style scoped>
</style>
