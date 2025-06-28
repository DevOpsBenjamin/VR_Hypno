<script setup lang="ts">
import { useNavigationStore } from '@/store/navigation'
import SessionList from './SessionList.vue';
import SessionEditor from './SessionEditor.vue';

const nav = useNavigationStore()
</script>

<template>
  <div class="p-4 overflow-y-auto h-full">
    <SessionList v-if="nav.path[1] === 'list'" />
    <SessionEditor v-else-if="nav.path[1] === 'edit'"/>
  </div>
</template>