
import { Session } from '@shared/domains/session/types';
import { Song } from '@shared/domains/song/types';
import { updateSession, getSession } from '@shared/domains/session/endpoints';
import { getSong } from '@shared/domains/song/endpoints';

export class EditorManager {
  currentSession: Session
  currentSong: Song
    
  //asynchronous loading of session and song
  async loadSession(sessionUid: string) {
    if (!sessionUid) return;
    
    try {
      const result = await getSession(sessionUid);
      if (result.success && result.data) {
        this.currentSession = result.data.session;        
        // Load song if session has one
        if (this.currentSession.info.song_uid) {
          await this.loadSong();
        }
      } else {
        throw new Error(result.error || 'Failed to load session');
      }
    } catch (e) {
      console.error('Error loading session:', e);
    }
  }
  async loadSong() {
    const songUid = this.currentSession.info.song_uid;
    if (!songUid) return;

    try {
      const result = await getSong(songUid);
      if (result.success && result.data) {
        this.currentSong = result.data.song;
      } else {
        throw new Error(result.error || 'Failed to load song');
      }
    } catch (e) {
      console.error('Error loading song:', e);
    }
  }
  async saveSession() {
    try {
      const result = await updateSession(this.currentSession);
      if (!result.success) {
        throw new Error(result.error || 'Failed to save session');
      }
    } catch (e) {
      console.error('Error saving session:', e);
    }
  }


  constructor() {
    this.currentSession = {
      uid: '',
      info: {
        name: '',
        song_uid: '',
        tracks: [],
        description: undefined,
        threejsConfig: undefined
      }
    };
    
    this.currentSong = {
      uid: '',
      info: {
        name: '',
        duration: 0
      }
    }
  }
}