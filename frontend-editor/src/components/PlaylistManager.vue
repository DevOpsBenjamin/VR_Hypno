<script setup lang="ts">
import { useNavigationStore } from '@/store/navigation'
import PlaylistList from '@/components/playlist/PlaylistList.vue'
import PlaylistEditor from '@/components/playlist/PlaylistEditor.vue'
import PlaylistPlayer from '@/components/playlist/PlaylistPlayer.vue'

const nav = useNavigationStore()
</script>

<template>
  <div class="p-4 overflow-y-auto h-full">
    <PlaylistList v-if="nav.path[2] === 'list'" />
    <PlaylistEditor v-else-if="nav.path[2] === 'edit'"/>
    <PlaylistPlayer v-else-if="nav.path[2] === 'player'"/>
  </div>
</template>