import type { ThreeJSConfig } from '../../types/threejs';

export interface SessionInfo {
  name: string;
  song: string; // Song ID
  threejsConfig?: ThreeJSConfig;
  // ...other session-level metadata
}

export interface Session {
  uid: string;
  info: SessionInfo;
}
