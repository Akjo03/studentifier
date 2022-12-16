use axum::{response::IntoResponse, response::Response, Json};
use serde::{Serialize, Deserialize};

pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub refresh_token: String,
} impl User {
    pub fn new(id: String, username: String, password_hash: String, salt: String, refresh_token: String) -> Self {
        Self {
            id,
            username,
            password_hash,
            salt,
            refresh_token,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
} impl LoginResponse {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
} impl IntoResponse for LoginResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
} impl RefreshResponse {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
} impl IntoResponse for RefreshResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutResponse {
    pub message: String,
} impl LogoutResponse {
    pub fn new(message: String) -> Self {
        Self {
            message,
        }
    }
} impl IntoResponse for LogoutResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}