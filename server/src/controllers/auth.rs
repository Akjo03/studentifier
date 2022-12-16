use crate::prelude::*;
use crate::models::auth::*;

use axum::Json;

pub async fn login(_request: Json<LoginRequest>) -> ApiResult<LoginResponse> {
    Ok(LoginResponse::new(
        "test".to_string(),
        "test".to_string(),
    ))
}

pub async fn refresh(_request: Json<RefreshRequest>) -> ApiResult<RefreshResponse> {
    Ok(RefreshResponse::new(
        "test".to_string(),
        "test".to_string(),
    ))
}

pub async fn logout(_request: Json<LogoutRequest>) -> ApiResult<LogoutResponse> {
    Ok(LogoutResponse::new("test".to_string()))
}