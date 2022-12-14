use axum::{response::IntoResponse, Json};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteInfo {
    pub path: String,
    pub allowed_methods: Vec<String>,
    pub description: String,
} impl RouteInfo {
    pub fn new(path: &str, allowed_methods: Vec<String>, description: &str) -> Self {
        Self {
            path: path.to_string(),
            allowed_methods,
            description: description.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoResponse {
    pub app_name: String,
    pub app_version: String,
    pub routes: Vec<RouteInfo>,
} impl InfoResponse {
    pub fn default() -> Self {
        Self {
            app_name: "studentifier-server".to_string(),
            app_version: "0.1.0".to_string(),
            routes: vec![
                RouteInfo::new("/api/v1/info", vec!["GET".to_string()], "Get information about this API."),
            ]
        }
    }
} impl IntoResponse for InfoResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}