import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

export default defineConfig({
  plugins: [vue()],
  build: {
    outDir: '../dist/editor',
    emptyOutDir: true,
  },
  server: {
    port: 5173
  },
});
