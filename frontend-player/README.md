# Frontend Player (Mode Simple)

Ce dossier contient le mode Player : HTML/JS minimal pour lire les scènes 3D.
- Ajoute ici tes scripts Three.js, gestion de fichiers, etc.

# VR Hypno Player - Technical Overview

## Architecture
- **Vue 3 + Composition API**: All UI is in single-file components, using `<script setup lang="ts">`.
- **Composables**: All logic is modularized (playlist, audio, XR, ThreeJS, CPU tracker) in `/src/composables`.
- **No global state**: State is local to the player, no Pinia/Vuex.
- **i18n**: All UI strings use the shared `t` function from `/shared/i18n.ts`.
- **Types**: All domain types are in `/shared/types.ts` and imported everywhere.
- **Tauri invoke**: All backend communication uses Tauri's invoke, never fetch/axios.
- **ThreeJS/XR**: Ready for VR/XR integration, with a dedicated canvas and composables.

## Contribution Guidelines
- All code, types, and comments must be in English.
- UI/UX strings must use the i18n `t` function.
- Always use composables for reusable logic.
- Add TODO/FIXME for any shortcuts or known issues.
- Test build (`npm run build:player`) after every major change.
- Prefer clarity and maintainability over cleverness.

## Next Steps
- [ ] Finalize XR/ThreeJS integration (see TODO in Player.vue and useXR/useThreeJS).
- [ ] Add more robust error handling and user feedback.
- [ ] Expand i18n coverage if needed.
- [ ] Add more unit tests and runtime checks.

## Project Structure
- `/frontend-player/src/Player.vue`: Main static player page.
- `/frontend-player/src/composables/`: All logic (playlist, audio, XR, ThreeJS, CPU tracker).
- `/shared/`: Types and helpers shared between player, editor, and backend.

---

For any new feature or refactor, always check `.github/instructions/Refactor.instructions.md` for standards.
