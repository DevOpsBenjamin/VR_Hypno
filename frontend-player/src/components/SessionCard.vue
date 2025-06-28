<template>
  <div
    class="w-[95%] mx-auto px-3 py-2 rounded-lg shadow transition cursor-pointer border border-brand-300 bg-brand-300 hover:bg-brand-500"
    :class="{
      'bg-brand-500 text-white shadow-lg border-brand-400': isSelected,
      'hover:scale-[1.01]': true
    }"
    @click="selectSession"
  >
    <span class="block text-sm font-semibold truncate">{{ session.info.name }}</span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { usePlayerManager } from '@/composables/usePlayerManager';
import type { Session } from '@shared/domains/session/types';

const props = defineProps<{ session: Session }>();
const playerManager = usePlayerManager();

const isSelected = computed(() => playerManager.currentSessionUid.value === props.session.uid);

function selectSession() {
  playerManager.currentSessionUid.value = props.session.uid;
}
</script>

<style scoped>
</style>
