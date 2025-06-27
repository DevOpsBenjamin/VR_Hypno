use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ThreeJSConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<ThreeJSObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera: Option<ThreeJSCamera>,
    // Add other scene-level config fields as needed
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ThreeJSObject {
    pub r#type: ThreeJSObjectType,
    pub position: [f32; 3],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<[f32; 3]>,
    // Add other object params as needed
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ThreeJSObjectType {
    Box,
    Sphere,
    Custom,
}

impl Default for ThreeJSObjectType {
    fn default() -> Self {
        ThreeJSObjectType::Box
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ThreeJSCamera {
    pub position: [f32; 3],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fov: Option<f32>,
}
