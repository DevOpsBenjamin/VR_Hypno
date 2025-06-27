use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
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
    let mut dir = app_data_dir(&app.config()).unwrap();
    dir.push("playlists");
    dir
}

#[tauri::command]
pub fn get_playlists(app: AppHandle) -> Result<Vec<Playlist>, String> {
    let dir = playlists_dir(&app);
    let mut playlists = Vec::new();
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path().join("info.json");
            if path.exists() {
                if let Ok(mut file) = File::open(&path) {
                    let mut content = String::new();
                    if file.read_to_string(&mut content).is_ok() {
                        if let Ok(info) = serde_json::from_str::<PlaylistInfo>(&content) {
                            if let Some(uid) = entry.file_name().to_str() {
                                playlists.push(Playlist { uid: uid.to_string(), info });
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(playlists)
}

#[tauri::command]
pub fn get_playlist(app: AppHandle, uid: String) -> Result<Playlist, String> {
    let path = playlists_dir(&app).join(&uid).join("info.json");
    let mut file = File::open(&path).map_err(|_| "Playlist not found")?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|_| "Read error")?;
    let info: PlaylistInfo = serde_json::from_str(&content).map_err(|_| "Parse error")?;
    Ok(Playlist { uid, info })
}

#[tauri::command]
pub fn create_playlist(app: AppHandle, name: String, repeat: bool, sessions: Vec<String>) -> Result<Playlist, String> {
    let uid = nanoid::nanoid!();
    let dir = playlists_dir(&app).join(&uid);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let info = PlaylistInfo { name, repeat, sessions, duration: None };
    let info_path = dir.join("info.json");
    let json = serde_json::to_string_pretty(&info).unwrap();
    let mut file = File::create(&info_path).map_err(|e| e.to_string())?;
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
    Ok(Playlist { uid, info })
}

#[tauri::command]
pub fn update_playlist(app: AppHandle, uid: String, info: PlaylistInfo) -> Result<(), String> {
    let dir = playlists_dir(&app).join(&uid);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let info_path = dir.join("info.json");
    let json = serde_json::to_string_pretty(&info).unwrap();
    let mut file = File::create(&info_path).map_err(|e| e.to_string())?;
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_playlist(app: AppHandle, uid: String) -> Result<(), String> {
    let dir = playlists_dir(&app).join(&uid);
    if dir.exists() {
        fs::remove_dir_all(&dir).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Playlist not found".into())
    }
}