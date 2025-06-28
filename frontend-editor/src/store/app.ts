import { defineStore } from 'pinia'

export const useAppStore = defineStore('app', {
  state: () => ({
    selectedPlaylistUid: null as string | null,
    selectedSessionUid: null as string | null,
  }),
  actions: {
    setPlaylist(uid: string | null) {
      this.selectedPlaylistUid = uid
    },
    setSession(uid: string | null) {
      this.selectedSessionUid = uid
    },
  },
  persist: {
    key: 'vr-hypno-app-store',
    storage: window.localStorage,
  },
})