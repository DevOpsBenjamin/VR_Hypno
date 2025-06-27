use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SongInfo {
    pub name: String,
    pub duration: Option<u64>,
    pub tags: Option<Vec<String>>,
    pub triggers: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Song {
    pub uid: String,
    pub info: SongInfo,
}

#[derive(Serialize)]
pub struct SongsData {
    pub songs: Vec<Song>,
}

#[derive(Serialize)]
pub struct SongData {
    pub song: Song,
}

#[derive(Serialize)]
pub struct AddSongData {
    pub song: Song,
}

#[derive(Serialize)]
pub struct SongAudioUrlData {
    pub url: String,
}
