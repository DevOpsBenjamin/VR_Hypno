use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use tauri::api::path::app_data_dir;
use tauri::AppHandle;

#[derive(Serialize, Deserialize, Clone)]
pub struct SongInfo {
    pub name: String,
    pub duration: Option<u64>,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Song {
    pub uid: String,
    pub info: SongInfo,
}

fn songs_dir(app: &AppHandle) -> PathBuf {
    let mut dir = app_data_dir(&app.config()).unwrap();
    dir.push("songs");
    dir
}

fn sessions_dir(app: &AppHandle) -> PathBuf {
    let mut dir = app_data_dir(&app.config()).unwrap();
    dir.push("sessions");
    dir
}

#[tauri::command]
pub fn get_songs(app: AppHandle) -> Result<Vec<Song>, String> {
    let dir = songs_dir(&app);
    let mut songs = Vec::new();
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path().join("info.json");
            if path.exists() {
                if let Ok(mut file) = File::open(&path) {
                    let mut content = String::new();
                    if file.read_to_string(&mut content).is_ok() {
                        if let Ok(info) = serde_json::from_str::<SongInfo>(&content) {
                            if let Some(uid) = entry.file_name().to_str() {
                                songs.push(Song { uid: uid.to_string(), info });
                            }
                        }
                    }
                }
            }
        }
    }
    // Sort by name
    songs.sort_by(|a, b| a.info.name.cmp(&b.info.name));
    Ok(songs)
}

#[tauri::command]
pub fn get_song(app: AppHandle, uid: String) -> Result<Song, String> {
    let path = songs_dir(&app).join(&uid).join("info.json");
    let mut file = File::open(&path).map_err(|_| "Song not found")?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|_| "Read error")?;
    let info: SongInfo = serde_json::from_str(&content).map_err(|_| "Parse error")?;
    Ok(Song { uid, info })
}

#[tauri::command]
pub fn add_song(app: AppHandle, info: SongInfo) -> Result<Song, String> {
    let uid = nanoid::nanoid!();
    let dir = songs_dir(&app).join(&uid);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let info_path = dir.join("info.json");
    let json = serde_json::to_string_pretty(&info).unwrap();
    let mut file = File::create(&info_path).map_err(|e| e.to_string())?;
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
    Ok(Song { uid, info })
}

#[tauri::command]
pub fn update_song(app: AppHandle, uid: String, info: SongInfo) -> Result<(), String> {
    let dir = songs_dir(&app).join(&uid);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let info_path = dir.join("info.json");
    let json = serde_json::to_string_pretty(&info).unwrap();
    let mut file = File::create(&info_path).map_err(|e| e.to_string())?;
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_song(app: AppHandle, uid: String) -> Result<(), String> {
    let song_dir = songs_dir(&app).join(&uid);
    if !song_dir.exists() {
        return Err("Song not found".into());
    }
    // Check if song is used in any session
    let sessions_dir = sessions_dir(&app);
    if let Ok(entries) = fs::read_dir(&sessions_dir) {
        for entry in entries.flatten() {
            let info_path = entry.path().join("info.json");
            if info_path.exists() {
                if let Ok(mut file) = File::open(&info_path) {
                    let mut content = String::new();
                    if file.read_to_string(&mut content).is_ok() {
                        if let Ok(session_info) = serde_json::from_str::<serde_json::Value>(&content) {
                            if let Some(song_uid) = session_info.get("song_uid").and_then(|s| s.as_str()) {
                                if song_uid == uid {
                                    return Err("Cannot delete song: it is being used in one or more sessions".into());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fs::remove_dir_all(&song_dir).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_song_audio_url(app: AppHandle, uid: String) -> Result<String, String> {
    let path = songs_dir(&app).join(&uid).join("audio.mp3");
    if path.exists() {
        // Return the absolute path as a string; use Tauri's convertFileSrc on the frontend
        path.to_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "Invalid path encoding".to_string())
    } else {
        Err("Audio file not found".into())
    }
}

#[tauri::command]
pub fn import_song_audio(app: AppHandle, uid: String, source_path: String) -> Result<(), String> {
    use std::fs::copy;
    let dest = songs_dir(&app).join(&uid).join("audio.mp3");
    std::fs::create_dir_all(dest.parent().unwrap()).map_err(|e| e.to_string())?;
    copy(&source_path, &dest).map_err(|e| e.to_string())?;
    Ok(())
}