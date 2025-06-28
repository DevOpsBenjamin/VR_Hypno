<script setup lang="ts">
import Header from './components/Header.vue'
import EditorManager from './components/EditorManager.vue'
import EditorSidebar from './components/EditorSidebar.vue'
</script>

<template>
  <div class="h-screen flex flex-col overflow-hidden">
    <!-- HEADER SECTION -->
    <div class="w-full h-20 flex-shrink-0 bg-brand-500">
      <Header />
    </div>

    <!-- CONTENT GRID -->
    <div class="flex-1 flex min-h-0">
      <!-- LEFT SIDEBAR -->
      <EditorSidebar />
      <!-- RIGHT CONTENT -->
      <div class="flex-1 min-w-0 bg-brand-50">
        <div class="h-full w-full">
          <EditorManager />
        </div>
      </div>
    </div>
  </div>
</template>