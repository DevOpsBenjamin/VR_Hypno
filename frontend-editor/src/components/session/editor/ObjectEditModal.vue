<template>
  <div class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-40 z-50" style="display: none;">
    <div class="bg-white rounded-xl shadow-lg p-8 min-w-[320px]">
      <div class="font-bold mb-2">{{ t('sessionObjectEditTitle') }}</div>
      <div class="text-brand-400 mb-4">[Object Edit Form Placeholder]</div>
      <button class="btn btn-primary" type="button">
        {{ t('commonClose') }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { t } from '@shared/utils/i18n';
</script>
