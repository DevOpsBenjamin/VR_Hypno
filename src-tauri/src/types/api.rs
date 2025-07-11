use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub error: Option<String>,
    pub data: Option<T>,
}
