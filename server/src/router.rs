use axum::{
    Router,
    routing::{get},
};
use tower_http::cors::{ Any, CorsLayer };

pub fn get_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any);

    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(cors)
}