use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use tauri::api::path::app_data_dir;
use tauri::AppHandle;

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionInfo {
    pub name: String,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    pub uid: String,
    pub info: SessionInfo,
}

fn sessions_dir(app: &AppHandle) -> PathBuf {
    let mut dir = app_data_dir(&app.config()).unwrap();
    dir.push("sessions");
    dir
}

fn playlists_dir(app: &AppHandle) -> PathBuf {
    let mut dir = app_data_dir(&app.config()).unwrap();
    dir.push("playlists");
    dir
}

#[tauri::command]
pub fn get_sessions(app: AppHandle) -> Result<Vec<Session>, String> {
    let dir = sessions_dir(&app);
    let mut sessions = Vec::new();
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path().join("info.json");
            if path.exists() {
                if let Ok(mut file) = File::open(&path) {
                    let mut content = String::new();
                    if file.read_to_string(&mut content).is_ok() {
                        if let Ok(info) = serde_json::from_str::<SessionInfo>(&content) {
                            if let Some(uid) = entry.file_name().to_str() {
                                sessions.push(Session { uid: uid.to_string(), info });
                            }
                        }
                    }
                }
            }
        }
    }
    // Sort by name
    sessions.sort_by(|a, b| a.info.name.cmp(&b.info.name));
    Ok(sessions)
}

#[tauri::command]
pub fn get_session(app: AppHandle, uid: String) -> Result<Session, String> {
    let path = sessions_dir(&app).join(&uid).join("info.json");
    let mut file = File::open(&path).map_err(|_| "Session not found")?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|_| "Read error")?;
    let info: SessionInfo = serde_json::from_str(&content).map_err(|_| "Parse error")?;
    Ok(Session { uid, info })
}

#[tauri::command]
pub fn create_session(app: AppHandle, info: SessionInfo) -> Result<Session, String> {
    let uid = nanoid::nanoid!();
    let dir = sessions_dir(&app).join(&uid);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let info_path = dir.join("info.json");
    let json = serde_json::to_string_pretty(&info).unwrap();
    let mut file = File::create(&info_path).map_err(|e| e.to_string())?;
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
    Ok(Session { uid, info })
}

#[tauri::command]
pub fn update_session(app: AppHandle, uid: String, info: SessionInfo) -> Result<(), String> {
    let dir = sessions_dir(&app).join(&uid);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let info_path = dir.join("info.json");
    let json = serde_json::to_string_pretty(&info).unwrap();
    let mut file = File::create(&info_path).map_err(|e| e.to_string())?;
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_session(app: AppHandle, uid: String) -> Result<(), String> {
    let session_dir = sessions_dir(&app).join(&uid);
    if !session_dir.exists() {
        return Err("Session not found".into());
    }
    // Check if session is used in any playlist
    let playlists_dir = playlists_dir(&app);
    if let Ok(entries) = fs::read_dir(&playlists_dir) {
        for entry in entries.flatten() {
            let info_path = entry.path().join("info.json");
            if info_path.exists() {
                if let Ok(mut file) = File::open(&info_path) {
                    let mut content = String::new();
                    if file.read_to_string(&mut content).is_ok() {
                        if let Ok(playlist_info) = serde_json::from_str::<serde_json::Value>(&content) {
                            if let Some(sessions) = playlist_info.get("sessions").and_then(|s| s.as_array()) {
                                if sessions.iter().any(|s| s.as_str() == Some(&uid)) {
                                    return Err("Cannot delete session: it is being used in one or more playlists".into());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fs::remove_dir_all(&session_dir).map_err(|e| e.to_string())?;
    Ok(())
}
