<script setup lang="ts">
import { useNavigationStore } from '../store/navigation'
import SessionManager from '@/components/session/SessionManager.vue'
import SongManager from '@/components/song/SongManager.vue'
import AssetManager from '@/components/asset/AssetManager.vue'
import PlaylistManager from '@/components/playlist/PlaylistManager.vue'

const nav = useNavigationStore()
</script>

<template>
  <div class="w-full h-full bg-gradient-to-br from-brand-100 via-brand-50 to-brand-200 rounded-xl shadow-lg">
    <PlaylistManager v-if="nav.path[0] === 'playlists'" />
    <SessionManager v-else-if="nav.path[0] === 'sessions'" />
    <SongManager v-else-if="nav.path[0] === 'songs'" />
    <AssetManager v-else-if="nav.path[0] === 'assets'" />
  </div>
</template>