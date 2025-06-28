use crate::types::threejs::ThreeJSConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionInfo {
    pub name: String,
    pub song_uid: String,
    pub description: Option<String>,
    pub tracks: Vec<Track>,
    pub threejs_config: Option<ThreeJSConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Track {
    pub id: String,
    pub objects: Vec<TrackObj>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TrackObj {
    pub obj_id: String,
    pub obj_type: String,
    pub spawn: f32,
    pub despawn: f32,
    pub asset_id: Option<String>,
    pub timeline: Vec<TrackTimelineEvent>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TrackTimelineEvent {
    pub start: f32,
    pub end: f32,
    pub action: String,
    // ...other fields
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    pub uid: String,
    pub info: SessionInfo,
}

#[derive(Serialize)]
pub struct SessionsData {
    pub sessions: Vec<Session>,
}

#[derive(Serialize)]
pub struct SessionData {
    pub session: Session,
}

#[derive(Serialize)]
pub struct CreateSessionData {
    pub session: Session,
}
