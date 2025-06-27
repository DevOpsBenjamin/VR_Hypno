---
applyTo: '**'
---

# Coding standards, domain knowledge, and preferences

## Coding Standards
- Use modern JavaScript/TypeScript (ES2020+), and Vue 3 Composition API for all new code.
- Prefer functional, composable, and modular code. Avoid monolithic files.
- Use clear, descriptive variable and function names (English for code, French for UI if needed).
- Always type variables and function signatures in TypeScript.
- Use async/await for asynchronous code, never raw Promises.
- Use const and let appropriately; avoid var.
- Prefer destructuring and spread/rest syntax for objects/arrays.
- Use single quotes for JS/TS, double quotes for HTML attributes.
- Use Prettier formatting defaults (2 spaces, trailing commas where valid, semicolons).
- Always check for null/undefined before accessing properties.
- Use explicit error handling (try/catch) for all async operations that can fail.
- Write code that is easy to test and maintain.

## Vue/Frontend Preferences
- Use single-file components (.vue) for all UI.
- Use <script setup lang="ts"> for all Vue components.
- No global state management (Pinia/Vuex) in the player; keep state local or use provide/inject if needed.
- Use composables for reusable logic (e.g., usePlaylist, useXR, useCPUTracker).
- Keep the player page static, no navigation, no router.
- UI must be responsive and accessible (ARIA where relevant).
- Use Tailwind CSS if present, otherwise scoped CSS in components.
- Use Tauri's invoke for all backend communication; never use fetch/axios for local APIs.

## Domain Knowledge
- The app is a VR/XR hypnosis experience editor/player.
- A playlist is a set of songs, each song has audio and a ThreeJS config for VR.
- The player loads a playlist, displays the current song, and provides basic controls.
- The editor is more complex, with timeline and 3D world editing.
- Data is stored in AppData/Local/{AppName} for Tauri, not in the project root.
- Shared types and API contracts must be in /shared and imported by both apps and backend.

## Preferences
- Avoid code duplication between editor and player; use /shared for types, helpers, and API contracts.
- Favor clarity and maintainability over cleverness.
- Add TODO/FIXME comments for any shortcuts or known issues.
- All code must compile and run before moving to the next step.
- Regularly test builds and runtime in both dev and prod modes.
- All code (function names, variables, comments, types) should be in English.
- UI/UX strings (what the user sees) should be in i18n format, using the `t` function for translations.