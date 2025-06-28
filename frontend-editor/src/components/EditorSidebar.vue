<template>
  <aside :class="[
    'flex-shrink-0 h-full min-h-0 flex flex-col bg-brand-100 shadow-xl transition-all duration-300', 
    navStore.menu_open ? 'w-15' : 'w-56'
    ]">
    <div class="flex items-center justify-center mt-4 mb-2">
      <button
        v-if="!navStore.menu_open"
        aria-label="Expand sidebar"
        class="flex items-center justify-center rounded-full bg-brand-200 hover:bg-brand-300 text-brand-700 shadow transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-brand-400 w-12 h-12"
        @click="toggleMenu"
      >
        <span class="text-4xl">{{ LeftEmoji }}</span>
      </button>
      <button
        v-else
        aria-label="Collapse sidebar"
        class="flex items-center justify-center rounded-full bg-brand-200 hover:bg-brand-300 text-brand-700 shadow transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-brand-400 w-12 h-12"
        @click="toggleMenu"
      >
        <span class="text-4xl">{{ RightEmoji }}</span>
      </button>
    </div>
    <nav class="flex-1 flex flex-col gap-2 mt-6">
      <button v-for="section in sections" :key="section.key"
        @click="selectSection(section.key)"
        :class="[
          'flex items-center gap-3 px-4 py-3 rounded-full font-bold transition-all',
          navStore.path[0] === section.key ? 'bg-brand-500 text-white shadow-lg scale-105' : 'bg-brand-200 text-brand-700 hover:bg-brand-300',
          navStore.menu_open ? 'justify-center px-2' : 'justify-start'
        ]"
        :title="section.label"
      >
        <span class="text-3xl">{{ section.icon }}</span>
        <span v-if="!navStore.menu_open" class="text-base">{{ section.label }}</span>
      </button>
    </nav>
  </aside>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { navigationTree, nav, NavigationPath } from '../utils/navigationTree'
import { useNavigationStore } from '../store/navigation'
import { BrainEmoji, MusicEmoji, MusicsEmoji, DiamondEmoji, FolderEmoji, RightEmoji, LeftEmoji} from '@shared/icons/emoji'

const navStore = useNavigationStore()

const sectionKeys = Object.keys(navigationTree) as Array<keyof typeof navigationTree>

const sections = computed(() =>
  sectionKeys.map(key => {
    let icon = FolderEmoji
    if (key === 'sessions') icon = BrainEmoji
    if (key === 'songs') icon = MusicEmoji
    if (key === 'playlists') icon = MusicsEmoji
    if (key === 'assets') icon = DiamondEmoji
    return { key, label: key.charAt(0).toUpperCase() + key.slice(1), icon }
  })
)

function selectSection(key: keyof typeof navigationTree) {
  navStore.navigateTo(nav[key].list as NavigationPath)
}

function toggleMenu() {
  navStore.toggleMenu()
}
</script>