<script setup lang="ts">
// TODO: Replace navigation logic with local state or provide/inject if needed
import SessionList from './SessionList.vue';
import SessionEditor from './SessionEditor.vue';

const view = 'list'; // FIXME: Replace with real navigation if needed
</script>

<template>
  <div class="p-4 overflow-y-auto h-full">
    <SessionList v-if="view === 'list'" />
    <SessionEditor v-else-if="view === 'edit'" />
  </div>
</template>
