<script setup lang="ts">
import { useNavigationStore } from '@/store/navigation'
import SongList from '@/components/song/SongList.vue'
import SongEditor from '@/components/song/SongEditor.vue'

const nav = useNavigationStore()
</script>

<template>
  <div class="p-4 overflow-y-auto h-full">
    <SongList v-if="nav.path[1] === 'list'" />
    <SongEditor v-else-if="nav.path[1] === 'edit'"/>
  </div>
</template>