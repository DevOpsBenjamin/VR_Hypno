import { defineConfig } from 'vite';

export default defineConfig({
  base: './',
  build: {
    outDir: '../dist/player',
    emptyOutDir: true,
  },
  server: {
    port: 5174
  },
});
