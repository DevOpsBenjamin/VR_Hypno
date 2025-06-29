<template>
  <div class="w-full">
    <!-- Compact header: zoom controls + time labels in one line -->
    <div class="flex items-center justify-between mb-2 text-xs">
      <!-- Left: Zoom controls -->
      <div class="flex items-center gap-2">
        <button 
          @click="zoomOut" 
          :disabled="zoomLevel <= 1"
          class="px-2 py-1 bg-brand-100 hover:bg-brand-200 disabled:opacity-50 disabled:cursor-not-allowed rounded"
          aria-label="Zoom out"
        >
          🔍−
        </button>
        <span class="text-brand-600 min-w-[50px] text-center font-mono">
          {{ Math.round(zoomLevel * 100) }}%
        </span>
        <button 
          @click="zoomIn" 
          :disabled="zoomLevel >= maxZoomLevel"
          class="px-2 py-1 bg-brand-100 hover:bg-brand-200 disabled:opacity-50 disabled:cursor-not-allowed rounded"
          aria-label="Zoom in"
        >
          🔍+
        </button>
        <button 
          @click="resetZoom"
          class="px-2 py-1 bg-brand-100 hover:bg-brand-200 rounded"
          aria-label="Reset zoom"
        >
          Fit
        </button>
      </div>
      
      <!-- Center: Current time -->
      <div class="font-bold text-brand-700 font-mono">
        {{ formatDuration(editor.currentTime) }}
      </div>
      
      <!-- Right: Time range -->
      <div class="text-brand-400 font-mono">
        {{ formatDuration(viewStart) }} - {{ formatDuration(viewEnd) }}
      </div>
    </div>
    
    <!-- Main timeline with waveform background -->
    <div class="flex items-center w-full relative">
      <!-- TODO: Waveform visualization background (like Audacity) -->
      <div class="absolute inset-0 h-10 bg-brand-50 rounded-lg overflow-hidden">
        <!-- Placeholder for future waveform canvas/svg -->
        <div class="w-full h-full bg-gradient-to-r from-brand-100 via-brand-50 to-brand-100 opacity-30"></div>
      </div>
      
      <input
        type="range"
        :min="viewStart"
        :max="viewEnd"
        :step="0.01"
        v-model.number="editor.currentTime"
        class="flex-1 relative z-10 bg-transparent rounded-lg appearance-none cursor-pointer focus:outline-none focus:ring-2 focus:ring-brand-400"
        :disabled="!editor.currentSong"
        aria-label="Timeline position"
        @wheel.prevent="handleWheel"
      />
    </div>
    
    <!-- Overview timeline (mini-map) -->
    <div class="mt-2 relative h-2 bg-brand-100 rounded overflow-hidden">
      <div 
        class="absolute h-full bg-brand-300 opacity-60"
        :style="overviewWindowStyle"
      ></div>
      <div 
        class="absolute h-full w-0.5 bg-brand-600"
        :style="overviewCurrentTimeStyle"
      ></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject, reactive, computed, watch, ref } from 'vue';
import { formatDuration } from '@shared/utils/format';
import { EditorManager } from '@/app/EditorManager';

const editor = reactive(inject<EditorManager>('editor')!);
const maxDuration = computed(() => editor.currentSong?.info.duration || 1);

// Zoom state
const zoomLevel = ref(1); // 1 = 100% (full song), 2 = 200% (half song), etc.
const zoomCenter = ref(0); // Center point of zoom in seconds

// Dynamic zoom limit based on song duration (allow zooming to ~5 second windows)
const maxZoomLevel = computed(() => {
  const duration = maxDuration.value;
  const minWindowSize = 5; // Minimum 5-second window
  return Math.max(10, Math.ceil(duration / minWindowSize));
});

// Calculate visible time window based on zoom
const viewStart = computed(() => {
  const duration = maxDuration.value;
  const windowSize = duration / zoomLevel.value;
  const start = Math.max(0, zoomCenter.value - windowSize / 2);
  return Math.min(start, duration - windowSize);
});

const viewEnd = computed(() => {
  const duration = maxDuration.value;
  const windowSize = duration / zoomLevel.value;
  return Math.min(viewStart.value + windowSize, duration);
});

// Zoom functions
const zoomIn = () => {
  if (zoomLevel.value < maxZoomLevel.value) {
    zoomCenter.value = editor.currentTime; // Zoom around current time
    zoomLevel.value = Math.min(maxZoomLevel.value, zoomLevel.value * 1.5);
  }
};

const zoomOut = () => {
  if (zoomLevel.value > 1) {
    zoomLevel.value = Math.max(1, zoomLevel.value / 1.5);
    if (zoomLevel.value === 1) {
      zoomCenter.value = maxDuration.value / 2; // Center on full song
    }
  }
};

const resetZoom = () => {
  zoomLevel.value = 1;
  zoomCenter.value = maxDuration.value / 2;
};

// Mouse wheel zoom
const handleWheel = (event: WheelEvent) => {
  if (event.ctrlKey || event.metaKey) {
    event.preventDefault();
    if (event.deltaY < 0) {
      zoomIn();
    } else {
      zoomOut();
    }
  }
};

// Clamp currentTime to valid range and keep it in view
watch(maxDuration, (val) => {
  if (editor.currentTime > val) {
    editor.currentTime = val;
  }
  if (zoomLevel.value === 1) {
    zoomCenter.value = val / 2;
  }
});

// Keep current time in view when zooming
watch([viewStart, viewEnd], () => {
  if (editor.currentTime < viewStart.value) {
    editor.currentTime = viewStart.value;
  } else if (editor.currentTime > viewEnd.value) {
    editor.currentTime = viewEnd.value;
  }
});

// No slider background style needed - removed blue gradient
// The slider is just for time selection, waveform provides visual context

// Overview timeline styles
const overviewWindowStyle = computed(() => {
  const duration = maxDuration.value;
  if (duration === 0) return { left: '0%', width: '100%' };
  
  const left = (viewStart.value / duration) * 100;
  const width = ((viewEnd.value - viewStart.value) / duration) * 100;
  return {
    left: `${left}%`,
    width: `${width}%`,
  };
});

const overviewCurrentTimeStyle = computed(() => {
  const duration = maxDuration.value;
  if (duration === 0) return { left: '0%' };
  
  const left = (editor.currentTime / duration) * 100;
  return {
    left: `${left}%`,
  };
});
</script>

<style scoped>
input[type='range'] {
  width: 100%;
  -webkit-appearance: none;
  appearance: none;
  background: transparent;
  height: 40px;
  border-radius: 12px;
}

input[type='range']::-webkit-slider-runnable-track {
  height: 40px;
  background: transparent;
  border-radius: 12px;
}

input[type='range']::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 10px;
  height: 40px;
  background: #2563eb;
  border-radius: 6px;
  box-shadow: 0 0 2px rgba(0,0,0,0.15);
  cursor: pointer;
  transition: background 0.2s;
  margin-top: 0;
}

input[type='range']:focus::-webkit-slider-thumb {
  background: #1d4ed8;
}

input[type='range']::-moz-range-thumb {
  width: 10px;
  height: 40px;
  background: #2563eb;
  border-radius: 6px;
  box-shadow: 0 0 2px rgba(0,0,0,0.15);
  cursor: pointer;
  transition: background 0.2s;
  border: none;
}

input[type='range']:focus::-moz-range-thumb {
  background: #1d4ed8;
}

input[type='range']::-moz-range-track {
  height: 40px;
  background: transparent;
  border-radius: 12px;
}

input[type='range']::-ms-thumb {
  width: 10px;
  height: 40px;
  background: #2563eb;
  border-radius: 6px;
  box-shadow: 0 0 2px rgba(0,0,0,0.15);
  cursor: pointer;
  transition: background 0.2s;
}

input[type='range']:focus::-ms-thumb {
  background: #1d4ed8;
}

input[type='range']:focus {
  outline: none;
}

input[type='range']:disabled::-webkit-slider-thumb,
input[type='range']:disabled::-moz-range-thumb,
input[type='range']:disabled::-ms-thumb {
  background: #cbd5e1;
  cursor: not-allowed;
}
</style>
