use axum::{
    Router,
    routing::{get, post},
};
use tower_http::cors::{ Any, CorsLayer };

pub fn get_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any);

    Router::new()
        .route("/api/v1/info", get(crate::controllers::info::info))
        .route("/api/v1/auth/login", post(crate::controllers::auth::login))
        .route("/api/v1/auth/refresh", post(crate::controllers::auth::refresh))
        .route("/api/v1/auth/logout", post(crate::controllers::auth::logout))
        .layer(cors)
}