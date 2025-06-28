import { defineStore } from 'pinia';
import { localeStoreState, localeStoreActions, sharedLocale } from '@shared/utils/localeStore';

export const useLocaleStore = defineStore('locale', {
  state: localeStoreState,
  actions: localeStoreActions,
  persist: {
    key: 'vr-hypno-locale-store',
    storage: window.localStorage,
  },
  // Synchronise le sharedLocale à l'init
  hydrate(state) {
    sharedLocale.lang = state.lang;
  },
});