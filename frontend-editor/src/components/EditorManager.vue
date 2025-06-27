<script setup lang="ts">
import { useNavigationStore } from '../store/navigation'
import SessionManager from '@/components/session/SessionManager.vue'
import SongManager from '@/components/song/SongManager.vue'
import AssetManager from '@/components/assets/AssetManager.vue'

const nav = useNavigationStore()
</script>

<template>
  <div class="w-full h-full bg-gradient-to-br from-brand-100 via-brand-50 to-brand-200 rounded-xl shadow-lg">
    <SessionManager v-if="nav.path[1] === 'sessions'" />
    <SongManager v-else-if="nav.path[1] === 'songs'" />
    <AssetManager v-else-if="nav.path[1] === 'assets'" />
  </div>
</template>