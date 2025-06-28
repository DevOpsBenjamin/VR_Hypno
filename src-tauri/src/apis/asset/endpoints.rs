use crate::apis::asset::types::*;
use std::fs;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;
use tauri::AppHandle;
use crate::apis::session::endpoints::sessions_dir;

pub fn assets_dir(app: &AppHandle) -> PathBuf {
    app_data_dir(&app.config())
        .unwrap()
        .join("assets")
}

pub fn get_assets_internal(app: AppHandle) -> Result<AssetsData, String> {
    let dir = assets_dir(&app);
    let entries = fs::read_dir(&dir).map_err(|e| e.to_string())?;
    let mut assets: Vec<Asset> = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let info_path = entry.path().join("info.json");
            if !info_path.exists() {
                return None;
            }
            let uid = entry.file_name().to_str()?.to_string();
            let content = fs::read_to_string(&info_path).ok()?;
            let info = serde_json::from_str::<AssetInfo>(&content).ok()?;
            Some(Asset { uid, info })
        })
        .collect();
    assets.sort_by(|a, b| a.info.name.cmp(&b.info.name));
    Ok(AssetsData { assets })
}

pub fn get_asset_internal(app: AppHandle, uid: String) -> Result<AssetData, String> {
    let path = assets_dir(&app).join(&uid).join("info.json");
    let content = fs::read_to_string(&path)
        .map_err(|_| "asset not found".to_string())?;
    let info = serde_json::from_str::<AssetInfo>(&content)
        .map_err(|_| "Invalid asset format".to_string())?;
    Ok(AssetData { asset: Asset { uid, info } })
}

pub fn update_asset_internal(app: AppHandle, uid: String, info: AssetInfo) -> Result<(), String> {
    let path = assets_dir(&app).join(&uid).join("info.json");
    if !path.exists() {
        return Err("asset not found".to_string());
    }
    let json = serde_json::to_string_pretty(&info)
        .map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

pub fn delete_asset_internal(app: AppHandle, uid: String) -> Result<(), String> {
    let asset_dir = assets_dir(&app).join(&uid);
    if !asset_dir.exists() {
        return Err("asset not found".to_string());
    }
    if is_asset_in_use_internal(&app, &uid)? {
        return Err("Cannot delete asset: it is being used in one or more sessions".to_string());
    }
    fs::remove_dir_all(&asset_dir).map_err(|e| e.to_string())
}

pub fn is_asset_in_use_internal(app: &AppHandle, uid: &str) -> Result<bool, String> {
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
        if let Some(asset_uid) = session_info.get("asset_uid").and_then(|s| s.as_str()) {
            if asset_uid == uid {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub fn get_asset_obj_internal(app: AppHandle, uid: String) -> Result<AssetUrlData, String> {
    let path = assets_dir(&app).join(&uid).join("audio.mp3");
    if !path.exists() {
        return Err("Audio file not found".to_string());
    }
    let url = path.to_str().ok_or_else(|| "Invalid path encoding".to_string())?.to_string();
    Ok(AssetUrlData { url })
}

pub fn import_asset_obj_internal(app: AppHandle, source_path: String) -> Result<AddAssetData, String> {
    let uid = nanoid::nanoid!();
    let dir = assets_dir(&app).join(&uid);
    let dest = dir.join("obj.glb");
    let source_name = PathBuf::from(&source_path);
    fs::create_dir_all(dest.parent().unwrap())
        .map_err(|e| e.to_string())?;
    fs::copy(&source_path, &dest)
        .map_err(|e| format!("Failed to copy obj file: {}", e))?;
    let info = AssetInfo {
        name: source_name.file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string(),
        tags: None,
    };
    let json = serde_json::to_string_pretty(&info)
        .map_err(|e| e.to_string())?;
    fs::write(dir.join("info.json"), json)
        .map_err(|e| e.to_string())?;
    Ok(AddAssetData { asset: Asset { uid, info } })
}