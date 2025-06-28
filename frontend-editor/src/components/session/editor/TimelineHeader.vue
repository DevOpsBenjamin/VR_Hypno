<template>
  <div class="flex items-center gap-2 mb-2 w-full min-h-[48px]">
    <input
      type="range"
      min="0"
      :max="editor.currentSong.info.duration"
      v-model.number="editor.currentTime"
      class="flex-1 h-4 accent-brand-500 bg-brand-100 rounded-lg appearance-none cursor-pointer focus:outline-none focus:ring-2 focus:ring-brand-400"
      :style="'min-width:200px; max-width:1000px;'"
      :disabled="!editor.currentSong"
      aria-label="Timeline position"
    />
    <div class="text-xs text-brand-400 min-w-[90px] text-right">
      {{ formatDuration(editor.currentTime) }} / {{ formatDuration(editor.currentSong.info.duration) }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject, reactive } from 'vue';
import { t } from '@shared/utils/i18n';
import { formatDuration } from '@shared/utils/format';
import { EditorManager } from '@/app/EditorManager';

const editor = reactive(inject<EditorManager>('editor')!);
</script>
