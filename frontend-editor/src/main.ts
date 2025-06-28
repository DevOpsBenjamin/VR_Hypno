import { createApp } from 'vue'
import App from './App.vue';
import { createPinia } from 'pinia'
import piniaPersistedstate from 'pinia-plugin-persistedstate'

import './style.css'
import { EditorManager } from '@/app/EditorManager';

const app = createApp(App)
const pinia = createPinia()
pinia.use(piniaPersistedstate)

// Because pinia sucks and doesn't support providing a store instance
const editor = new EditorManager()
app.provide<EditorManager>('editor',  editor)

app.use(pinia)
app.mount('#app')
