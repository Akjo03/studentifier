use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Failed to start server: {0}")]
    ServerStartError(String),
}

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    InternalServerError(String),
} impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            ApiError::BadRequest(message) => (StatusCode::BAD_REQUEST, format!("Failed to handle request: {}", message)),
            ApiError::InternalServerError(message) => (StatusCode::INTERNAL_SERVER_ERROR, format!("An internal server error occurred: {}", message)),
        };

        let json = json!({
            "error": error_message
        });

        (status_code, Json(json)).into_response()
    }
}