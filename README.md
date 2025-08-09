# VR_Hypno
VR Hypno est une application innovante qui combine la technologie de réalité virtuelle avec les techniques d'hypnose. Cette application offre une expérience immersive aux utilisateurs recherchant des séances.

## Objectif
Créer une application immersive d'hypnose en réalité virtuelle, avec deux modes :
- **Mode Éditeur** : Application Vue.js complète pour créer et éditer des scènes 3D synchronisées avec l'audio.
- **Mode Player** : Interface HTML/JS minimaliste pour lire les scènes 3D et proposer des sessions d'hypnose immersives.

## Technologies
- **Backend** : Rust (Tauri) avec accès au système de fichiers
- **Frontend** : Vue.js (éditeur) et HTML/JS/Three.js (player)
- **Distribution** : Exécutable autonome (Tauri)

## Fonctionnalités principales
- Import et édition de fichiers audio et scènes 3D
- Timeline synchronisée avec l'audio
- Placement d'objets 3D et effets temporisés
- Export/sauvegarde et lecture de scènes
- Support WebXR pour les casques VR

## Structure du projet
- **frontend-editor** : Application Vue 3 pour créer et éditer des scènes.
- **frontend-player** : Lecteur 3D minimaliste basé sur HTML/JS/Three.js.
- **shared** : Types et utilitaires communs aux deux frontends et au backend.
- **src-tauri** : Backend Rust pour l'accès au système de fichiers et l'emballage de l'application.
- **poc** : Prototypes et expérimentations.

## Développement
1. Installer les dépendances : `npm install`
2. Lancer le mode développement : `npm run dev`
3. Construire l'éditeur : `npm run build:editor`
4. Construire le lecteur : `npm run build:player`

## TODO / Améliorations
- Finaliser l'interface de timeline et l'édition d'objets dans l'éditeur.
- Remplacer la commande `copy` dans les scripts de build par une alternative multiplateforme.
- Ajouter des tests unitaires et une intégration continue.
- Renforcer la gestion des erreurs et les messages utilisateur.
- Documenter davantage la structure du projet et les conventions de code.

## URL du projet
https://github.com/DevOpsBenjamin/VR_Hypno
