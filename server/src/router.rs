use axum::{
    Router,
    routing::{get},
};
use tower_http::cors::{ Any, CorsLayer };

pub fn get_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any);

    Router::new()
        .route("/api/v1/info", get(crate::controllers::info::info))
        .layer(cors)
}