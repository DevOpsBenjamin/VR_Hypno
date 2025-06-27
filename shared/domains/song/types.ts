export interface SongInfo {
  name: string;
  duration: number;
  audioUrl: string;
  tags?: string[];
  triggers?: string[];
}

export interface Song {
  uid: string;
  info: SongInfo;
}
