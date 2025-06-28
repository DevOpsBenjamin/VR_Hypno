import { defineStore } from 'pinia'

export const useLocaleStore = defineStore('locale', {
  state: () => ({
    lang: 'en' as 'en' | 'fr'
  }),
  actions: {
    setLocale(lang: 'en' | 'fr') {
      this.lang = lang
    },
  },
  persist: {
    key: 'vr-hypno-local-store',
    storage: window.localStorage,
  },
})