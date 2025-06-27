use crate::types::api::ApiResponse;
use crate::apis::playlist::types::*;
use crate::apis::playlist::endpoints::*;
use tauri::AppHandle;

#[tauri::command]
pub fn get_playlists(app: AppHandle) -> ApiResponse<PlaylistsData> {
    match get_playlists_internal(app) {
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
pub fn get_playlist(app: AppHandle, uid: String) -> ApiResponse<PlaylistData> {
    match get_playlist_internal(app, uid) {
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
pub fn create_playlist(app: AppHandle, name: String, repeat: bool, sessions: Vec<String>) -> ApiResponse<CreatePlaylistData> {
    match create_playlist_internal(app, name, repeat, sessions) {
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
pub fn update_playlist(app: AppHandle, uid: String, info: PlaylistInfo) -> ApiResponse<()> {
    match update_playlist_internal(app, uid, info) {
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
pub fn delete_playlist(app: AppHandle, uid: String) -> ApiResponse<()> {
    match delete_playlist_internal(app, uid) {
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
