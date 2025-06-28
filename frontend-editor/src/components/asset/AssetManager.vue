<script setup lang="ts">
import { useNavigationStore } from '@/store/navigation'
import AssetList from '@/components/asset/AssetList.vue'
import AssetEditor from '@/components/asset/AssetEditor.vue'

const nav = useNavigationStore()
</script>

<template>
  <div class="p-4 overflow-y-auto h-full">
    <AssetList v-if="nav.path[1] === 'list'" />
    <AssetEditor v-else-if="nav.path[1] === 'edit'"/>
  </div>
</template>