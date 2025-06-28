import { defineStore } from 'pinia'
import type { NavigationPath } from '../utils/navigationTree'
import { nav } from '../utils/navigationTree'

export const useNavigationStore = defineStore('navigation', {
  state: () => ({
    path: nav.playlists.list as NavigationPath,
    options: {} as any,
    menu_open: false as boolean,
  }),
  actions: {
    navigateTo(path: NavigationPath, options?: any) {
      this.path = [...path] as NavigationPath
      this.options = options ?? {}
    },
    toggleMenu() {
      this.menu_open = !this.menu_open
    },
  },
  persist: {
    key: 'vr-hypno-navigation-store',
    storage: window.localStorage,
  }
}) 