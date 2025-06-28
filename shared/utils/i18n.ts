import { sharedLocale } from '@shared/utils/localeStore'; // À adapter selon l'app (editor/player)

export type Locale = keyof typeof messages;
export type MessageKey = keyof typeof messages['en'];

export function t(key: MessageKey): string {
  return messages[sharedLocale.lang.value][key] || key;
}

// Messages i18n partagés
export const messages = {
  en: {
    appTitle: 'VR Hypno',
    player: 'Player',
    editor: 'Editor',
    playlists: 'Playlists',
    addPlaylist: 'Add Playlist',
    createPlaylist: 'Create Playlist',
    loadingPlaylist: 'Loading playlists...',
    playlistName: 'Playlist name',
    repeat: 'Repeat',
    noSessions:  'No sessions yet. Click + to create one!',
    noPlaylists: 'No playlists yet. Click + to create one!',
    loading: 'Loading ...',
    unknownError: 'Unknown error',
    cancel: 'Cancel',
    yes: 'Yes',
    no: 'No',
    save: 'Save',
    sessions: 'Sessions',
    songs: 'Songs',
    addSong: 'Add Song',
    songName: 'Song name',
    noSongs: 'No songs',
    noSongsDesc: 'Add your first song to get started',
    duration: 'Duration',
    tags: 'Tags',
    back: 'Back',
    edit: 'Edit',
    delete: 'Delete',
    confirmTitle: 'Confirm',
    confirmDeleteSong: 'Are you sure you want to delete this song?',
    confirmDeleteSession: 'Are you sure you want to delete this session?',
    retry: 'Retry',
    confirmDeletePlaylist: 'Are you sure you want to delete this playlist?',
    addSession: 'Add Session',
    selectSession: 'Select Session'
  },
  fr: {
    appTitle: 'VR Hypno',
    player: 'Lecteur',
    editor: 'Éditeur',
    playlists: 'Playlists',
    addPlaylist: 'Ajouter une playlist',
    createPlaylist: 'Créer une playlist',
    loadingPlaylist:  'Chargement des playlists...',
    playlistName: 'Nom de la playlist',
    repeat: 'Répéter',
    noSessions: 'Aucune session pour l\'instant. Cliquez sur + pour en créer une !',
    noPlaylists: 'Aucune playlist pour l\'instant. Cliquez sur + pour en créer une !',
    loading: 'Chargement ...',
    unknownError: 'Erreur inconnue',
    cancel: 'Annuler',
    yes: 'Oui',
    no: 'Non',
    save: 'Enregistrer',
    sessions: 'Sessions',
    songs: 'Chansons',
    addSong: 'Ajouter une chanson',
    songName: 'Nom de la chanson',
    noSongs: 'Aucune chanson',
    noSongsDesc: 'Ajoutez votre première chanson pour commencer',
    duration: 'Durée',
    tags: 'Tags',
    back: 'Retour',
    edit: 'Éditer',
    delete: 'Supprimer',
    confirmTitle: 'Confirmer',
    confirmDeleteSong: 'Êtes-vous sûr de vouloir supprimer cette chanson ?',
    confirmDeleteSession: 'Êtes-vous sûr de vouloir supprimer cette session ?',
    retry: 'Réessayer',
    confirmDeletePlaylist: 'Êtes-vous sûr de vouloir supprimer cette playlist ?',
    addSession: 'Ajouter une session',
    selectSession: 'Sélectionner une session'
  }
};