#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod apis {
    pub mod song;
    pub mod sessions;
    pub mod playlist;
}


fn main() {
    tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![
            // Song
            apis::song::get_songs,
            apis::song::get_song,
            apis::song::add_song,
            apis::song::update_song,
            apis::song::delete_song,
            apis::song::get_song_audio_url,
            apis::song::import_song_audio,
            // Sessions
            apis::sessions::get_sessions,
            apis::sessions::get_session,
            apis::sessions::create_session,
            apis::sessions::update_session,
            apis::sessions::delete_session,
            // Playlist
            apis::playlist::get_playlists,
            apis::playlist::get_playlist,
            apis::playlist::create_playlist,
            apis::playlist::update_playlist,
            apis::playlist::delete_playlist,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}