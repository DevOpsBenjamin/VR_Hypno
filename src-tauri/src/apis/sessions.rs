use crate::types::threejs::ThreeJSConfig;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;
use tauri::AppHandle;

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionInfo {
    pub name: String,
    pub song_uid: String,
    pub description: Option<String>,
    pub threejs_config: Option<ThreeJSConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    pub uid: String,
    pub info: SessionInfo,
}

fn sessions_dir(app: &AppHandle) -> PathBuf {
    app_data_dir(&app.config())
        .unwrap()
        .join("sessions")
}

fn playlists_dir(app: &AppHandle) -> PathBuf {
    app_data_dir(&app.config())
        .unwrap()
        .join("playlists")
}

#[tauri::command]
pub fn get_sessions(app: AppHandle) -> Result<Vec<Session>, String> {
    let dir = sessions_dir(&app);
    
    let entries = fs::read_dir(&dir).map_err(|e| e.to_string())?;
    
    let mut sessions: Vec<Session> = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let info_path = entry.path().join("info.json");
            if !info_path.exists() {
                return None;
            }
            
            let uid = entry.file_name().to_str()?.to_string();
            let content = fs::read_to_string(&info_path).ok()?;
            let info = serde_json::from_str::<SessionInfo>(&content).ok()?;
            
            Some(Session { uid, info })
        })
        .collect();
    
    sessions.sort_by(|a, b| a.info.name.cmp(&b.info.name));
    Ok(sessions)
}

#[tauri::command]
pub fn get_session(app: AppHandle, uid: String) -> Result<Session, String> {
    let path = sessions_dir(&app).join(&uid).join("info.json");
    
    let content = fs::read_to_string(&path)
        .map_err(|_| "Session not found")?;
    
    let info = serde_json::from_str::<SessionInfo>(&content)
        .map_err(|_| "Invalid session format")?;
    
    Ok(Session { uid, info })
}

#[tauri::command]
pub fn create_session(app: AppHandle, info: SessionInfo) -> Result<Session, String> {
    let uid = nanoid::nanoid!();
    let dir = sessions_dir(&app).join(&uid);
    
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    
    let json = serde_json::to_string_pretty(&info)
        .map_err(|e| e.to_string())?;
    
    fs::write(dir.join("info.json"), json)
        .map_err(|e| e.to_string())?;
    
    Ok(Session { uid, info })
}

#[tauri::command]
pub fn update_session(app: AppHandle, uid: String, info: SessionInfo) -> Result<(), String> {
    let path = sessions_dir(&app).join(&uid).join("info.json");
    
    if !path.exists() {
        return Err("Session not found".to_string());
    }
    
    let json = serde_json::to_string_pretty(&info)
        .map_err(|e| e.to_string())?;
    
    fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_session(app: AppHandle, uid: String) -> Result<(), String> {
    let session_dir = sessions_dir(&app).join(&uid);
    
    if !session_dir.exists() {
        return Err("Session not found".to_string());
    }
    
    // Check if session is used in any playlist
    if is_session_in_use(&app, &uid)? {
        return Err("Cannot delete session: it is being used in one or more playlists".to_string());
    }
    
    fs::remove_dir_all(&session_dir).map_err(|e| e.to_string())
}

fn is_session_in_use(app: &AppHandle, uid: &str) -> Result<bool, String> {
    let playlists_dir = playlists_dir(app);
    let entries = fs::read_dir(&playlists_dir).map_err(|e| e.to_string())?;
    
    for entry in entries.filter_map(|e| e.ok()) {
        let info_path = entry.path().join("info.json");
        if !info_path.exists() {
            continue;
        }
        
        let content = fs::read_to_string(&info_path).map_err(|e| e.to_string())?;
        let playlist_info: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| e.to_string())?;
        
        if let Some(sessions) = playlist_info.get("sessions").and_then(|s| s.as_array()) {
            if sessions.iter().any(|s| s.as_str() == Some(uid)) {
                return Ok(true);
            }
        }
    }
    
    Ok(false)
}
