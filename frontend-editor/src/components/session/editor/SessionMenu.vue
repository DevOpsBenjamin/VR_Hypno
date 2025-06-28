<template>
  <!-- Session Name + Description + Save Button -->
  <div class="flex flex-col gap-2 mb-4">
    <div class="flex items-center gap-2">
      <label class="w-20 font-semibold text-sm text-gray-700" for="session-name">Nom :</label>
      <input
        id="session-name"
        v-model="editor.currentSession.info.name"
        type="text"
        class="flex-1 border rounded p-1 text-base"
        :placeholder="t('sessionHeaderTitle')"
        aria-label="Session name"
      />
      <button
        class="ml-2 p-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition disabled:opacity-60"
        @click="save"
        type="button"
      >
        {{ t('save') }}
      </button>
    </div>
    <div class="flex items-center gap-2">
      <label class="w-20 font-semibold text-sm text-gray-700" for="session-desc">Description :</label>
      <input
        id="session-desc"
        v-model="editor.currentSession.info.description"
        type="text"
        class="flex-1 border rounded p-1 text-base"
        :placeholder="t('description')"
        aria-label="Session description"
      />
    </div>
  </div>

  <!-- Properties Section -->
  <div>
    <h2 class="font-semibold text-lg mb-2">Properties</h2>
    <!-- Properties content will go here -->
  </div>
</template>

<script setup lang="ts">
import { computed, inject, reactive } from 'vue';
import { t } from '@shared/utils/i18n';
import { EditorManager } from '@/app/EditorManager';

const editor = reactive(inject<EditorManager>('editor')!)

//METHOD PART
async function save() {
  await editor.saveSession();
}
</script>
