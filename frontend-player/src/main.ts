import { appWindow } from '@tauri-apps/api/window';

const isDev = import.meta.env.DEV;

const app = document.getElementById('app');
if (app) {
  app.innerHTML = `<h1>VR Hypno - Mode Player</h1>
  <button id='to-editor'>Basculer en mode éditeur</button>`;
}

const btn = document.getElementById('to-editor');
if (btn) {
  btn.addEventListener('click', async () => {
    if (isDev) {
      await appWindow.eval('window.location.replace("http://localhost:5173")');
    } else {
      await appWindow.eval('window.location.replace("/editor/index.html")');
    }
  });
}
