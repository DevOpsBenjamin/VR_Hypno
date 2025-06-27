import { createApp } from 'vue';
import Player from './Player.vue';

const isDev = import.meta.env && import.meta.env.DEV;

function goToEditor() {
  if (isDev) {
    window.location.replace("http://localhost:5173");
  } else {
    window.location.replace("/editor/index.html");
  }
}

const btn = document.getElementById('to-editor');
if (btn) {
  btn.addEventListener('click', goToEditor);
}

createApp(Player).mount('#app');