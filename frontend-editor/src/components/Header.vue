<script setup lang="ts">
import { t } from '@shared/utils/i18n'
import { useAppStore } from '../store/app'
import { useNavigationStore } from '../store/navigation'
import { nav, NavigationPath } from '../utils/navigationTree'
import { HeartEmoji } from '@shared/icons/emoji'
const appStore = useAppStore()
const navStore = useNavigationStore()
</script>

<template>
  <header class="fixed top-0 left-0 w-full bg-gradient-to-r from-brand-400 via-brand-200 to-brand-400 shadow-md z-10 border-b border-brand-200">
    <div class="max-w mx-auto px-2">
      <div class="flex justify-between items-center h-20">
        <div class="flex items-center space-x-4">
          <span class="inline-flex items-center justify-center w-12 h-12 rounded-full bg-brand-100 shadow-lg">
            <span class="h-12 items-center text-4xl">{{ HeartEmoji }}</span>
          </span>
          <button 
            @click="navStore.navigateTo(nav.player.playlist.list as NavigationPath)"
            :class="[
              'btn',
              'rounded-full',
              'bg-brand-500',
              'text-white',
              navStore.path[0] === 'player' ? 'ring-2 ring-brand-300 scale-105' : 'bg-brand-200 text-brand-700',
              'hover:bg-brand-600',
              'transition-all duration-150',
              'text-base px-6 py-2 shadow-lg font-bold tracking-wide uppercase'
            ]"
          >
            {{ t('player') }}
          </button>
          <button 
            :disabled="navStore.path[0] === 'editor'"
            @click="navStore.path[0] === 'editor' ? null : navStore.navigateTo(nav.editor.sessions.list as NavigationPath)"
            :class="[
              'btn',
              'rounded-full',
              'bg-brand-500',
              'text-white',
              navStore.path[0] === 'editor' ? 'ring-2 ring-brand-300 scale-105 opacity-60 cursor-default' : 'bg-brand-200 text-brand-700',
              'hover:bg-brand-600',
              'transition-all duration-150',
              'text-base px-6 py-2 shadow-lg font-bold tracking-wide uppercase'
            ]"
          >
          {{ t('editor') }}
          </button>
        </div>
        <div class="flex items-center space-x-2 ml-4">
          <button @click="appStore.setLocale('en')" :class="[appStore.locale === 'en' ? 'ring-2 ring-brand-400' : '', 'rounded-full p-1 transition']" title="English">
            <img src="/flag-gb.svg" alt="GB" class="w-7 h-5 rounded shadow-sm" />
          </button>
          <button @click="appStore.setLocale('fr')" :class="[appStore.locale === 'fr' ? 'ring-2 ring-brand-400' : '', 'rounded-full p-1 transition']" title="Français">
            <img src="/flag-fr.svg" alt="FR" class="w-7 h-5 rounded shadow-sm" />
          </button>
        </div>
      </div>
    </div>
  </header>
</template>