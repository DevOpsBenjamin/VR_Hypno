use crate::types::api::ApiResponse;
use crate::apis::asset::types::*;
use crate::apis::asset::endpoints::*;
use tauri::AppHandle;

#[tauri::command]
pub fn get_assets(app: AppHandle) -> ApiResponse<AssetsData> {
    match get_assets_internal(app) {
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
pub fn get_asset(app: AppHandle, uid: String) -> ApiResponse<AssetData> {
    match get_asset_internal(app, uid) {
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
pub fn update_asset(app: AppHandle, uid: String, info: AssetInfo) -> ApiResponse<()> {
    match update_asset_internal(app, uid, info) {
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
pub fn delete_asset(app: AppHandle, uid: String) -> ApiResponse<()> {
    match delete_asset_internal(app, uid) {
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
pub fn import_asset_obj(app: AppHandle, source_path: String) -> ApiResponse<AddAssetData> {
    match import_asset_obj_internal(app, source_path) {
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
pub fn get_asset_obj(app: AppHandle, uid: String) -> ApiResponse<AssetUrlData> {
    match get_asset_obj_internal(app, uid) {
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
