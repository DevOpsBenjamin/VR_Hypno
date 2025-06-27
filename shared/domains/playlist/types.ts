export interface PlaylistInfo {
  name: string;
  repeat: boolean;
  sessions: string[]; // Array of session IDs
  duration?: number;
}

export interface Playlist {
  uid: string;
  info: PlaylistInfo;
}
