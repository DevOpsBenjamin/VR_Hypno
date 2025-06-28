<template>
  <!-- Session Name + Save Button -->
  <div class="flex items-center mb-4 gap-2">
    <input
      v-model="sessionName"
      type="text"
      class="flex-1 border rounded px-2 py-1 text-base"
      :placeholder="t('session.name')"
      aria-label="Session name"
    />
    <button
      class="ml-2 px-4 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition"
      @click="onSave"
      type="button"
    >
      {{ t("common.save") }}
    </button>
  </div>

  <!-- Properties Section -->
  <div>
    <h2 class="font-semibold text-lg mb-2">{{ t("session.properties") }}</h2>
    <!-- Properties content will go here -->
  </div>
</template>

<script setup lang="ts">
import { t } from "@shared/utils/i18n";
</script>
