use axum::{response::IntoResponse, Json};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct RouteInfo {
    path: String,
    allowed_methods: Vec<String>,
    description: String,
} impl RouteInfo {
    #[allow(dead_code)]
    pub fn new(path: String, allowed_methods: Vec<String>, description: String) -> Self {
        Self {
            path,
            allowed_methods,
            description,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoResponse {
    app_name: String,
    app_version: String,
    routes: Vec<RouteInfo>,
} impl IntoResponse for InfoResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
} impl InfoResponse {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            app_name: "studentifier-server".to_string(),
            app_version: "0.1.0".to_string(),
            routes: vec![
                RouteInfo::new(
                    "/api/v1/info".to_string(),
                    vec![
                        "GET".to_string()
                    ],
                    "Returns information about the application.".to_string(),
                ),
                RouteInfo::new(
                    "/api/v1/auth/login".to_string(),
                    vec![
                        "POST".to_string()
                    ],
                    "Logs in a user using the username and password. Sends back a JWT access and refresh token.".to_string(),
                ),
                RouteInfo::new(
                    "/api/v1/auth/refresh".to_string(),
                    vec![
                        "POST".to_string()
                    ],
                    "Refreshes the JWT access token using the JWT refresh token.".to_string(),
                ),
                RouteInfo::new(
                    "/api/v1/auth/register".to_string(),
                    vec![
                        "POST".to_string()
                    ],
                    "Registers a new user. Sends back a JWT access and refresh token.".to_string(),
                ),
                RouteInfo::new(
                    "/api/v1/auth/logout".to_string(),
                    vec![
                        "POST".to_string()
                    ],
                    "Logs out a user using the JWT refresh token.".to_string(),
                ),
            ],
        }
    }
}