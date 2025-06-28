import type { ThreeJSConfig } from '../../types/threejs';

export interface SessionInfo {
    name: string;
    song_uid: string;
    tracks: Track[];
    description?: string;  // Optional description for the session
    threejsConfig?: ThreeJSConfig;
}

export interface Track {
  id: string;
  objects: TrackObj[];
} 

export interface TrackObj {
  obj_id: string;
  obj_type: string;
  spawn: number;
  despawn: number;
  asset_id?: string;
  timeline: TrackTimelineEvent[];
}

export interface TrackTimelineEvent {
  action: string;
  start: number;
  end: number;
} 

// For UI/backend usage:
export interface Session {
  uid: string;
  info: SessionInfo;
}