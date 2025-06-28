// Objet partagé pour la langue courante
import { ref } from 'vue';

export const sharedLocale = {
  lang: ref<'en' | 'fr'>('en')
};

export type Locale = 'en' | 'fr';

export const localeStoreState = () => ({
  lang: 'en' as Locale,
});

export const localeStoreActions = {
  setLocale(this: { lang: Locale }, lang: Locale) {
    this.lang = lang;
    sharedLocale.lang.value = lang; 
  },
};