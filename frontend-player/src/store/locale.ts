import { defineStore } from 'pinia';
import { localeStoreState, localeStoreActions, sharedLocale } from '@shared/utils/localeStore';

export const useLocaleStore = defineStore('player', {
  state: localeStoreState,
  actions: localeStoreActions,
  persist: {
    key: 'vr-player-locale-store',
    storage: window.localStorage,
  },
  // Synchronise le sharedLocale à l'init
  hydrate(state) {
    sharedLocale.lang.value = state.lang;
  },
});