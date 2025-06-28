use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AssetInfo {
    pub name: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Asset {
    pub uid: String,
    pub info: AssetInfo,
}

#[derive(Serialize)]
pub struct AssetsData {
    pub assets: Vec<Asset>,
}

#[derive(Serialize)]
pub struct AssetData {
    pub asset: Asset,
}

#[derive(Serialize)]
pub struct AddAssetData {
    pub asset: Asset,
}

#[derive(Serialize)]
pub struct AssetUrlData {
    pub url: String,
}
