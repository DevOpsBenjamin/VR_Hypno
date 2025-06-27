use crate::apis::song::types::*;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;
use tauri::AppHandle;
use crate::apis::session::endpoints::sessions_dir;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::formats::FormatOptions;
use symphonia::core::meta::MetadataOptions;
use symphonia::default::get_probe;

pub fn songs_dir(app: &AppHandle) -> PathBuf {
    app_data_dir(&app.config())
        .unwrap()
        .join("songs")
}

pub fn get_songs_internal(app: AppHandle) -> Result<SongsData, String> {
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
    Ok(SongsData { songs })
}

pub fn get_song_internal(app: AppHandle, uid: String) -> Result<SongData, String> {
    let path = songs_dir(&app).join(&uid).join("info.json");
    let content = fs::read_to_string(&path)
        .map_err(|_| "Song not found".to_string())?;
    let info = serde_json::from_str::<SongInfo>(&content)
        .map_err(|_| "Invalid song format".to_string())?;
    Ok(SongData { song: Song { uid, info } })
}

pub fn update_song_internal(app: AppHandle, uid: String, info: SongInfo) -> Result<(), String> {
    let path = songs_dir(&app).join(&uid).join("info.json");
    if !path.exists() {
        return Err("Song not found".to_string());
    }
    let json = serde_json::to_string_pretty(&info)
        .map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

pub fn delete_song_internal(app: AppHandle, uid: String) -> Result<(), String> {
    let song_dir = songs_dir(&app).join(&uid);
    if !song_dir.exists() {
        return Err("Song not found".to_string());
    }
    if is_song_in_use_internal(&app, &uid)? {
        return Err("Cannot delete song: it is being used in one or more sessions".to_string());
    }
    fs::remove_dir_all(&song_dir).map_err(|e| e.to_string())
}

pub fn is_song_in_use_internal(app: &AppHandle, uid: &str) -> Result<bool, String> {
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

pub fn get_song_audio_url_internal(app: AppHandle, uid: String) -> Result<SongAudioUrlData, String> {
    let path = songs_dir(&app).join(&uid).join("audio.mp3");
    if !path.exists() {
        return Err("Audio file not found".to_string());
    }
    let url = path.to_str().ok_or_else(|| "Invalid path encoding".to_string())?.to_string();
    Ok(SongAudioUrlData { url })
}

pub fn import_song_audio_internal(app: AppHandle, source_path: String) -> Result<AddSongData, String> {
    let uid = nanoid::nanoid!();
    let dir = songs_dir(&app).join(&uid);
    let dest = dir.join("audio.mp3");
    let source_name = PathBuf::from(&source_path);
    fs::create_dir_all(dest.parent().unwrap())
        .map_err(|e| e.to_string())?;
    fs::copy(&source_path, &dest)
        .map_err(|e| format!("Failed to copy audio file: {}", e))?;
    let duration = get_song_duration(&dest)
        .map_err(|e| format!("Failed to get song duration: {}", e))?;
    let info = SongInfo {
        name: source_name.file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string(),
        duration,
        tags: None,
        triggers: None,
    };
    let json = serde_json::to_string_pretty(&info)
        .map_err(|e| e.to_string())?;
    fs::write(dir.join("info.json"), json)
        .map_err(|e| e.to_string())?;
    Ok(AddSongData { song: Song { uid, info } })
}

pub fn get_song_duration(path: &PathBuf) -> Result<Option<u64>, String> {
    if !path.exists() {
        return Ok(None);
    }
    let file = File::open(path).map_err(|e| e.to_string())?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let probed = get_probe().format(
        &Default::default(),
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    );
    if let Ok(probed) = probed {
        let format = probed.format;
        if let Some(track) = format.default_track() {
            let params = &track.codec_params;
            if let (Some(n_frames), Some(sample_rate)) = (params.n_frames, params.sample_rate) {
                return Ok(Some(n_frames as u64 / sample_rate as u64));
            }
        }
    }
    Ok(None)
}
