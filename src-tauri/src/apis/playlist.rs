use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;
use tauri::AppHandle;

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

fn playlists_dir(app: &AppHandle) -> PathBuf {
    app_data_dir(&app.config())
        .unwrap()
        .join("playlists")
}

#[tauri::command]
pub fn get_playlists(app: AppHandle) -> Result<Vec<Playlist>, String> {
    let dir = playlists_dir(&app);
    
    let entries = fs::read_dir(&dir).map_err(|e| e.to_string())?;
    
    let mut playlists: Vec<Playlist> = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let info_path = entry.path().join("info.json");
            if !info_path.exists() {
                return None;
            }
            
            let uid = entry.file_name().to_str()?.to_string();
            let content = fs::read_to_string(&info_path).ok()?;
            let info = serde_json::from_str::<PlaylistInfo>(&content).ok()?;
            
            Some(Playlist { uid, info })
        })
        .collect();
    
    // Sort by name for consistency
    playlists.sort_by(|a, b| a.info.name.cmp(&b.info.name));
    
    Ok(playlists)
}

#[tauri::command]
pub fn get_playlist(app: AppHandle, uid: String) -> Result<Playlist, String> {
    let path = playlists_dir(&app).join(&uid).join("info.json");
    
    let content = fs::read_to_string(&path)
        .map_err(|_| "Playlist not found")?;
    
    let info = serde_json::from_str::<PlaylistInfo>(&content)
        .map_err(|_| "Invalid playlist format")?;
    
    Ok(Playlist { uid, info })
}

#[tauri::command]
pub fn create_playlist(app: AppHandle, name: String, repeat: bool, sessions: Vec<String>) -> Result<Playlist, String> {
    let uid = nanoid::nanoid!();
    let dir = playlists_dir(&app).join(&uid);
    
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    
    let info = PlaylistInfo { 
        name, 
        repeat, 
        sessions, 
        duration: None 
    };
    
    let json = serde_json::to_string_pretty(&info)
        .map_err(|e| e.to_string())?;
    
    fs::write(dir.join("info.json"), json)
        .map_err(|e| e.to_string())?;
    
    Ok(Playlist { uid, info })
}

#[tauri::command]
pub fn update_playlist(app: AppHandle, uid: String, info: PlaylistInfo) -> Result<(), String> {
    let path = playlists_dir(&app).join(&uid).join("info.json");
    
    // Verify playlist exists
    if !path.exists() {
        return Err("Playlist not found".to_string());
    }
    
    let json = serde_json::to_string_pretty(&info)
        .map_err(|e| e.to_string())?;
    
    fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_playlist(app: AppHandle, uid: String) -> Result<(), String> {
    let dir = playlists_dir(&app).join(&uid);
    
    if !dir.exists() {
        return Err("Playlist not found".to_string());
    }
    
    fs::remove_dir_all(&dir).map_err(|e| e.to_string())
}