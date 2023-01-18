use axum::{response::IntoResponse, Json};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub id: String,
    pub username: String,
    pub password: String,
    pub salt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    access_token: String,
    refresh_token: String,
} impl IntoResponse for LoginResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
} impl LoginResponse {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
} impl IntoResponse for RefreshRequest {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshResponse {
    access_token: String,
    refresh_token: String,
} impl IntoResponse for RefreshResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
} impl RefreshResponse {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub role: String,
} impl IntoResponse for RegisterRequest {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    access_token: String,
    refresh_token: String,
} impl IntoResponse for RegisterResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
} impl RegisterResponse {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
} impl IntoResponse for LogoutRequest {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutResponse {
    message: String,
} impl IntoResponse for LogoutResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
} impl LogoutResponse {
    pub fn new(message: String) -> Self {
        Self {
            message,
        }
    }
}
