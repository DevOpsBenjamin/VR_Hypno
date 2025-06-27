use crate::types::threejs::ThreeJSConfig;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct SessionInfo {
    pub name: String,
    pub song_uid: String,
    pub description: Option<String>,
    pub threejs_config: Option<ThreeJSConfig>,
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
