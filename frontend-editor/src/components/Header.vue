<template>
  <header class="fixed top-0 left-0 w-full bg-gradient-to-r from-brand-400 via-brand-200 to-brand-400 shadow-md z-10 border-b border-brand-200">
    <div class="max-w mx-auto px-2">
      <div class="flex justify-between items-center h-20">
        <div class="flex items-center space-x-4">
          <span class="inline-flex items-center justify-center w-12 h-12 rounded-full bg-brand-100 shadow-lg">
            <span class="h-12 items-center text-4xl">{{ HeartEmoji }}</span>
          </span>
        </div>
        <div class="flex items-center space-x-2 ml-4">
          <button @click="setLocale('en')" :class="[localeStore.lang === 'en' ? 'ring-2 ring-brand-400' : '', 'rounded-full p-1 transition']" title="English">
            <img src="/flag-gb.svg" alt="GB" class="w-13 h-8 rounded shadow-sm" />
          </button>
          <button @click="setLocale('fr')" :class="[localeStore.lang === 'fr' ? 'ring-2 ring-brand-400' : '', 'rounded-full p-1 transition']" title="Français">
            <img src="/flag-fr.svg" alt="FR" class="w-13 h-8 rounded shadow-sm" />
          </button>
        </div>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { useLocaleStore } from '@/store/locale'
import { HeartEmoji } from '@shared/icons/emoji'
const localeStore = useLocaleStore()

function setLocale(lang: 'en' | 'fr') {
  localeStore.setLocale(lang)
}
</script>