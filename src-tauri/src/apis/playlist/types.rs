use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct PlaylistInfo {
    pub name: String,
    pub repeat: bool,
    pub sessions: Vec<String>,
    pub duration: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Playlist {
    pub uid: String,
    pub info: PlaylistInfo,
}


#[derive(Serialize)]
pub struct PlaylistsData {
    pub playlists: Vec<Playlist>,
}

#[derive(Serialize)]
pub struct PlaylistData {
    pub playlist: Playlist,
}

#[derive(Serialize)]
pub struct CreatePlaylistData {
    pub playlist: Playlist,
}
