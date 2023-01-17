//! Definition of all error types for this application.

use std::fmt::Display;

use axum::{response::IntoResponse, Json, http::StatusCode};
use serde_json::json;

#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Failed to start server: {0}")]
    ServerStartError(String),
    #[error("Failed to parse query response from SurrealDB: {0}")]
    QueryResponseParseError(String),
    #[error("Failed to send query to SurrealDB: {0}")]
    QuerySendError(String),
    #[error("Failed to execute query on SurrealDB: {0}")]
    QueryExecutionError(String),
    #[error("Failed to connect to SurrealDB: {0}")]
    DatabaseConnectionError(String),
    #[error("Connection to SurrealDB is invalid: {0}")]
    DatabaseInvalidConnectionError(String),
    #[error("Failed to create SurrealDB client: {0}")]
    DatabaseClientCreationError(String),
} impl AppError {
    #[allow(dead_code)]
    pub fn log(self) -> Self {
        log::error!("{}", self);
        self
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ApiError {
    ServerError(String),
    BadRequest(String),
} impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ServerError(message) => write!(f, "An error occurred while processing your request on the server: {}", message),
            Self::BadRequest(message) => write!(f, "An error occurred while parsing your request: {}", message),
        }
    }
} impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_message) = match self {
            Self::ServerError(message) => (StatusCode::INTERNAL_SERVER_ERROR, format!("An error occurred while processing your request on the server: {}", message)),
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, format!("An error occurred while parsing your request: {}", message)),
        };
        (status_code, Json(json!({"error": error_message}))).into_response()
    }
} impl ApiError {
    #[allow(dead_code)]
    pub fn log(self) -> Self {
        log::error!("{}", self);
        self
    }
}