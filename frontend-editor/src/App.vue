<script setup lang="ts">
import { useNavigationStore } from './store/navigation'
import Header from './components/Header.vue'
import PlayerManager from './components/PlayerManager.vue'
import EditorManager from './components/EditorManager.vue'
import EditorSidebar from './components/EditorSidebar.vue'
import { useI18n, Locale } from '../../shared/utils/i18n';
import { ref } from 'vue';

const nav = useNavigationStore()
const locale = ref<Locale>('fr'); // or 'en', can be dynamic
const t = useI18n(locale);
</script>

<template>
  <div class="h-screen flex flex-col overflow-hidden">
    <!-- HEADER SECTION -->
    <div class="w-full h-20 flex-shrink-0 bg-brand-500">
      <Header />
    </div>

    <!-- CONTENT GRID -->
    <div class="flex-1 flex min-h-0">
      <!-- LEFT SIDEBAR -->
      <EditorSidebar v-if="nav.path[0] === 'editor'" />
      <!-- RIGHT CONTENT -->
      <div class="flex-1 min-w-0 bg-brand-50">
        <div class="h-full w-full">
          <PlayerManager v-if="nav.path[0] === 'player'" />
          <EditorManager v-else-if="nav.path[0] === 'editor'" />
        </div>
      </div>
    </div>
  </div>
</template>