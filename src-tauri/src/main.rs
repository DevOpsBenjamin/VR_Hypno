#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use tauri::{App, AppHandle, Manager};

mod apis {
    pub mod song;
    pub mod sessions;
    pub mod playlist;
}

fn app_data_dir(app: &AppHandle) -> PathBuf {
    tauri::api::path::app_data_dir(&app.config()).unwrap()
}

fn setup_default_data(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let app_data = app_data_dir(&app.app_handle());
    
    // Créer le dossier app_data s'il n'existe pas
    fs::create_dir_all(&app_data)?;
    
    // Vérifier si c'est la première exécution (dossier vide ou fichier marqueur)
    let marker_file = app_data.join(".initialized");
    if marker_file.exists() {
        println!("App data already initialized");
        return Ok(());
    }
    
    // Extraire les données par défaut
    extract_embedded_data(&app_data)?;
    
    // Créer le fichier marqueur
    fs::write(marker_file, "1")?;
    
    println!("Default data extracted successfully");
    Ok(())
}

// Alternative: Embarquer les données directement dans le binaire
fn extract_embedded_data(target_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Inclure le ZIP dans le binaire à la compilation
    const DEFAULT_DATA: &[u8] = include_bytes!("../resources/ressource-pack.zip");
    
    // Créer un lecteur ZIP depuis les bytes
    let reader = std::io::Cursor::new(DEFAULT_DATA);
    let mut archive = zip::ZipArchive::new(reader)?;
    
    // Extraire comme avant
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = target_dir.join(file.name());
        
        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                fs::create_dir_all(p)?;
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialiser les données par défaut
            if let Err(e) = setup_default_data(app) {
                eprintln!("Failed to setup default data: {}", e);
                // Décider si on veut crasher ou continuer sans données par défaut
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Song
            apis::song::get_songs,
            apis::song::get_song,
            apis::song::add_song,
            apis::song::update_song,
            apis::song::delete_song,
            apis::song::get_song_audio_url,
            apis::song::import_song_audio,
            // Sessions
            apis::sessions::get_sessions,
            apis::sessions::get_session,
            apis::sessions::create_session,
            apis::sessions::update_session,
            apis::sessions::delete_session,
            // Playlist
            apis::playlist::get_playlists,
            apis::playlist::get_playlist,
            apis::playlist::create_playlist,
            apis::playlist::update_playlist,
            apis::playlist::delete_playlist,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}