import type { ThreeJSConfig } from '../../types/threejs';

export interface SessionInfo {
    name: string;
    song_uid: string;
    description?: string;
    threejsConfig?: ThreeJSConfig;
}
  
// Pour l'usage UI/backend :
export interface Session {
  uid: string;
  info: SessionInfo;
} 