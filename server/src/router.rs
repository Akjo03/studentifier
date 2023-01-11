use crate::controllers;
use axum::{
    routing::{
        get, post
    },
    Router
};
use tower_http::cors::{
    Any,
    CorsLayer
};

pub fn get_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any);

    Router::new()
        .layer(cors)
        .route("/api/v1/info", get(controllers::info::info))
        .route("/api/v1/auth/login", post(controllers::auth::login))
        .route("/api/v1/auth/refresh", post(controllers::auth::refresh))
        .route("/api/v1/auth/register", post(controllers::auth::register))
        .route("/api/v1/auth/logout", post(controllers::auth::logout))
        .route("/api/v1/persons/all", get(controllers::persons::get_all_persons))
}