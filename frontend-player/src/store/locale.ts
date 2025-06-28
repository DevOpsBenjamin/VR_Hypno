import { defineStore } from 'pinia';
import { localeStoreState, localeStoreActions } from '@shared/utils/localeStore';

export const useLocaleStore = defineStore('player', {
  state: localeStoreState,
  actions: localeStoreActions,
  persist: {
    key: 'vr-player-locale-store',
    storage: window.localStorage,
  },
});