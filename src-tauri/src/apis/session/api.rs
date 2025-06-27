use crate::types::api::ApiResponse;
use crate::apis::session::types::*;
use crate::apis::session::endpoints::*;
use tauri::AppHandle;

#[tauri::command]
pub fn get_sessions(app: AppHandle) -> ApiResponse<SessionsData> {
    match get_sessions_internal(app) {
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
pub fn get_session(app: AppHandle, uid: String) -> ApiResponse<SessionData> {
    match get_session_internal(app, uid) {
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
pub fn create_session(app: AppHandle, info: SessionInfo) -> ApiResponse<CreateSessionData> {
    match create_session_internal(app, info) {
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
pub fn update_session(app: AppHandle, uid: String, info: SessionInfo) -> ApiResponse<()> {
    match update_session_internal(app, uid, info) {
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
pub fn delete_session(app: AppHandle, uid: String) -> ApiResponse<()> {
    match delete_session_internal(app, uid) {
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
