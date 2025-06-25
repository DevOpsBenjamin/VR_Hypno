import { defineConfig } from 'vite';

export default defineConfig({
  build: {
    outDir: '../dist/player',
    emptyOutDir: true,
  },
  server: {
    port: 5174
  },
});
