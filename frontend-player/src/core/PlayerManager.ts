import { getPlaylist } from "@shared/domains/playlist/endpoints";
import { Playlist } from "@shared/domains/playlist/types";
import { getSession } from "@shared/domains/session/endpoints";
import { Session } from "@shared/domains/session/types";
import { Ref, ref } from "vue";

export class PlayerManager {
  public sessions: Session[] = [];

  //ref for wathching changes in the UI
  public currentSessionUid: Ref<string, string> = ref<string>("");  
  public isInitialized = ref(false); // <- juste pour notifier l'UI

  // private fields
  private playlist: Playlist | null = null;

  async initialize(uid: string) {
    if (this.isInitialized.value) {
      console.warn("PlayerManager is already initialized.");
      return;
    }

    try {
      const result = await getPlaylist(uid);
      if (result?.success) {
        this.playlist = result.data?.playlist as Playlist;
      }
    } catch (e) {
      console.error(`Failed to load playlist ${uid}:`, e);
      return;
    }
    console.log("Playlist loaded:", uid);
    if (!this.playlist || !this.playlist.info.sessions || this.playlist.info.sessions.length === 0) {
      console.warn("No sessions found in the playlist.");
      return;
    }
    const sessionUids = this.playlist.info.sessions;
    console.log("Sessions found in the playlist:", this.playlist.info.sessions);
    try {
      for (const sessionUid of sessionUids) {
        const result = await getSession(sessionUid);
        if (!result?.success || !result.data?.session) {
          throw new Error(
            `Session data is missing for sessionUid: ${sessionUid}`
          );
        }
        const loadedSession = result.data.session as Session;
        if (loadedSession) {
          this.sessions.push(loadedSession as Session);
          console.log("Session loaded:", loadedSession.uid);
        }
      }
      
      this.currentSessionUid = ref<string>(this.sessions[0].uid);
    } catch (e) {
      console.error(`Failed to load a session`, e);
      return;
    }

    //Playlist is loaded
    this.isInitialized.value = true;
    console.log("isInitialized:", this.isInitialized.value);
  }
}

/*




export function usePlayerManager(playlistId: string) {
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

*/
