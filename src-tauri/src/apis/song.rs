use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;
use tauri::AppHandle;

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

fn songs_dir(app: &AppHandle) -> PathBuf {
    app_data_dir(&app.config())
        .unwrap()
        .join("songs")
}

fn sessions_dir(app: &AppHandle) -> PathBuf {
    app_data_dir(&app.config())
        .unwrap()
        .join("sessions")
}

#[tauri::command]
pub fn get_songs(app: AppHandle) -> Result<Vec<Song>, String> {
    let dir = songs_dir(&app);
    
    let entries = fs::read_dir(&dir).map_err(|e| e.to_string())?;
    
    let mut songs: Vec<Song> = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let info_path = entry.path().join("info.json");
            if !info_path.exists() {
                return None;
            }
            
            let uid = entry.file_name().to_str()?.to_string();
            let content = fs::read_to_string(&info_path).ok()?;
            let info = serde_json::from_str::<SongInfo>(&content).ok()?;
            
            Some(Song { uid, info })
        })
        .collect();
    
    songs.sort_by(|a, b| a.info.name.cmp(&b.info.name));
    Ok(songs)
}

#[tauri::command]
pub fn get_song(app: AppHandle, uid: String) -> Result<Song, String> {
    let path = songs_dir(&app).join(&uid).join("info.json");
    
    let content = fs::read_to_string(&path)
        .map_err(|_| "Song not found")?;
    
    let info = serde_json::from_str::<SongInfo>(&content)
        .map_err(|_| "Invalid song format")?;
    
    Ok(Song { uid, info })
}

#[tauri::command]
pub fn add_song(app: AppHandle, info: SongInfo) -> Result<Song, String> {
    let uid = nanoid::nanoid!();
    let dir = songs_dir(&app).join(&uid);
    
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    
    let json = serde_json::to_string_pretty(&info)
        .map_err(|e| e.to_string())?;
    
    fs::write(dir.join("info.json"), json)
        .map_err(|e| e.to_string())?;
    
    Ok(Song { uid, info })
}

#[tauri::command]
pub fn update_song(app: AppHandle, uid: String, info: SongInfo) -> Result<(), String> {
    let path = songs_dir(&app).join(&uid).join("info.json");
    
    if !path.exists() {
        return Err("Song not found".to_string());
    }
    
    let json = serde_json::to_string_pretty(&info)
        .map_err(|e| e.to_string())?;
    
    fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_song(app: AppHandle, uid: String) -> Result<(), String> {
    let song_dir = songs_dir(&app).join(&uid);
    
    if !song_dir.exists() {
        return Err("Song not found".to_string());
    }
    
    // Check if song is used in any session
    if is_song_in_use(&app, &uid)? {
        return Err("Cannot delete song: it is being used in one or more sessions".to_string());
    }
    
    fs::remove_dir_all(&song_dir).map_err(|e| e.to_string())
}

fn is_song_in_use(app: &AppHandle, uid: &str) -> Result<bool, String> {
    let sessions_dir = sessions_dir(app);
    let entries = fs::read_dir(&sessions_dir).map_err(|e| e.to_string())?;
    
    for entry in entries.filter_map(|e| e.ok()) {
        let info_path = entry.path().join("info.json");
        if !info_path.exists() {
            continue;
        }
        
        let content = fs::read_to_string(&info_path).map_err(|e| e.to_string())?;
        let session_info: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| e.to_string())?;
        
        if let Some(song_uid) = session_info.get("song_uid").and_then(|s| s.as_str()) {
            if song_uid == uid {
                return Ok(true);
            }
        }
    }
    
    Ok(false)
}

#[tauri::command]
pub fn get_song_audio_url(app: AppHandle, uid: String) -> Result<String, String> {
    let path = songs_dir(&app).join(&uid).join("audio.mp3");
    
    if !path.exists() {
        return Err("Audio file not found".to_string());
    }
    
    path.to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Invalid path encoding".to_string())
}

#[tauri::command]
pub fn import_song_audio(app: AppHandle, uid: String, source_path: String) -> Result<(), String> {
    let dest = songs_dir(&app).join(&uid).join("audio.mp3");
    
    fs::create_dir_all(dest.parent().unwrap())
        .map_err(|e| e.to_string())?;
    
    fs::copy(&source_path, &dest)
        .map_err(|e| format!("Failed to copy audio file: {}", e))?;
    
    Ok(())
}