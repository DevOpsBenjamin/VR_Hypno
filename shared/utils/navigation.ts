export function isDev(): boolean {
  return window.location.hostname === 'localhost';
}

export function goToEditor() {
  if (isDev()) {
    window.location.href = "http://localhost:5173";
  } else {
    window.location.href = "/editor/index.html";
  }
}

export function goToPlayer() {
  if (isDev()) {
    window.location.href = "http://localhost:5174";
  } else {
    window.location.href = "/player/index.html";
  }
}