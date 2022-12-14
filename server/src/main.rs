mod error;
mod prelude;

mod router;

mod controllers;
mod models;
mod util;

use crate::prelude::*;

use std::net::SocketAddr;
use axum::Server;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger
    simple_logger::init_with_level(log::Level::Info).unwrap();

    // Check connection to database
    // ...
    
    // Get router
    let router = router::get_router();

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    log::info!("Server listening on http://{}/...", addr);
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .map_err(|err| AppError::ServerStartError(err.to_string()))?;

    Ok(())
}