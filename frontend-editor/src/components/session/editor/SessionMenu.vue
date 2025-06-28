<template>
  <!-- Session Name + Save Button -->
  <div class="flex items-center mb-4 gap-2">
    <input
      v-model="sessionNameProxy"
      type="text"
      class="flex-1 border rounded px-2 py-1 text-base"
      :placeholder="t('sessionHeaderTitle')"
      aria-label="Session name"
      :disabled="loading || saving || !session"
    />
    <button
      class="ml-2 px-4 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition disabled:opacity-60"
      @click="emit('save')"
      type="button"
      :disabled="loading || saving || !session"
    >
      {{ t('save') }}
    </button>
  </div>

  <!-- Properties Section -->
  <div>
    <h2 class="font-semibold text-lg mb-2">Properties</h2>
    <!-- Properties content will go here -->
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { t } from '@shared/utils/i18n';
import type { Session } from '@shared/domains/session/types';

const props = defineProps<{
  session: Session | null;
  loading: boolean;
  saving: boolean;
}>();
const emit = defineEmits<{ (e: 'save'): void }>();

// Proxy for two-way binding on session name
const sessionNameProxy = computed({
  get: () => props.session?.info.name ?? '',
  set: (val: string) => {
    if (props.session) props.session.info.name = val;
  },
});
</script>
