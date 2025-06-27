import { ref, onMounted, onUnmounted } from 'vue';

export function useCPUTracker() {
  const cpuUsage = ref<number | null>(null);
  let cpuInterval: number | null = null;

  function startTracking() {
    let last = performance.now();
    let frames = 0;
    cpuInterval = window.setInterval(() => {
      const now = performance.now();
      frames++;
      if (now - last > 1000) {
        cpuUsage.value = Math.round((frames / ((now - last) / 1000)) * 100) / 100;
        frames = 0;
        last = now;
      }
    }, 100);
  }

  function stopTracking() {
    if (cpuInterval) clearInterval(cpuInterval);
    cpuInterval = null;
  }

  onMounted(startTracking);
  onUnmounted(stopTracking);

  return { cpuUsage };
}
