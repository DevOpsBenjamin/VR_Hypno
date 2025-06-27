use crate::types::api::ApiResponse;
use crate::apis::song::types::*;
use crate::apis::song::endpoints::*;
use tauri::AppHandle;

#[tauri::command]
pub fn get_songs(app: AppHandle) -> ApiResponse<SongsData> {
    match get_songs_internal(app) {
        Ok(data) => ApiResponse {
            success: true,
            error: None,
            data: Some(data),
        },
        Err(err) => ApiResponse {
            success: false,
            error: Some(err),
            data: None,
        },
    }
}

#[tauri::command]
pub fn get_song(app: AppHandle, uid: String) -> ApiResponse<SongData> {
    match get_song_internal(app, uid) {
        Ok(data) => ApiResponse {
            success: true,
            error: None,
            data: Some(data),
        },
        Err(err) => ApiResponse {
            success: false,
            error: Some(err),
            data: None,
        },
    }
}

#[tauri::command]
pub fn update_song(app: AppHandle, uid: String, info: SongInfo) -> ApiResponse<()> {
    match update_song_internal(app, uid, info) {
        Ok(_) => ApiResponse {
            success: true,
            error: None,
            data: None,
        },
        Err(err) => ApiResponse {
            success: false,
            error: Some(err),
            data: None,
        },
    }
}

#[tauri::command]
pub fn delete_song(app: AppHandle, uid: String) -> ApiResponse<()> {
    match delete_song_internal(app, uid) {
        Ok(_) => ApiResponse {
            success: true,
            error: None,
            data: None,
        },
        Err(err) => ApiResponse {
            success: false,
            error: Some(err),
            data: None,
        },
    }
}

#[tauri::command]
pub fn import_song_audio(app: AppHandle, source_path: String) -> ApiResponse<AddSongData> {
    match import_song_audio_internal(app, source_path) {
        Ok(data) => ApiResponse {
            success: true,
            error: None,
            data: Some(data),
        },
        Err(err) => ApiResponse {
            success: false,
            error: Some(err),
            data: None,
        },
    }
}

#[tauri::command]
pub fn get_song_audio_url(app: AppHandle, uid: String) -> ApiResponse<SongAudioUrlData> {
    match get_song_audio_url_internal(app, uid) {
        Ok(data) => ApiResponse {
            success: true,
            error: None,
            data: Some(data),
        },
        Err(err) => ApiResponse {
            success: false,
            error: Some(err),
            data: None,
        },
    }
}
