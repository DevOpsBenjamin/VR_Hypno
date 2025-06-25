/// <reference types="vite/client" />
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