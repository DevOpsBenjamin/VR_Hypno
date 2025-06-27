import { ref, Ref, onMounted } from 'vue';
import type { Playlist } from '@shared/playlist/types';
import type { Session } from '@shared/session/types';
import type { Song } from '@shared/song/types';
import type { ThreeJSConfig } from '@shared/threejs/types';

export function usePlaylist(playlistId: string) {
  const playlist = ref<Playlist | null>(null);
  const currentSession = ref<Session | null>(null);
  const currentSong = ref<Song | null>(null);
  const currentThreeJSConfig = ref<ThreeJSConfig | null>(null);
  const loading = ref(true);
  const error = ref<string | null>(null);

  async function fetchPlaylist() {
    loading.value = true;
    error.value = null;
    try {
      // Charge le playlist
      const result = await invoke<Playlist>('get_playlist', { id: playlistId });
      playlist.value = result;
      // Charge la première session si dispo
      const sessionId = result.info.sessions?.[0];
      if (sessionId) {
        const session = await invoke<Session>('get_session', { id: sessionId });
        currentSession.value = session;
        currentThreeJSConfig.value = session.info.threejsConfig || null;
        // Charge la song de la session
        if (session.info.song) {
          const song = await invoke<Song>('get_song', { id: session.info.song });
          currentSong.value = song;
        } else {
          currentSong.value = null;
        }
      } else {
        currentSession.value = null;
        currentSong.value = null;
        currentThreeJSConfig.value = null;
      }
    } catch (e: any) {
      error.value = e.message || 'Failed to load playlist/session/song';
    } finally {
      loading.value = false;
    }
  }

  onMounted(fetchPlaylist);

  return { playlist, currentSession, currentSong, currentThreeJSConfig, loading, error, fetchPlaylist };
}
