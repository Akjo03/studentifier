mod error;
mod prelude;

mod controllers;
mod models;
mod util;

mod router;

use std::net::ToSocketAddrs;

use crate::prelude::*;
use crate::router::get_router;
use crate::util::surrealdb;

use axum::Server;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    simple_logger::init_with_level(log::Level::Info).unwrap();

    // Get the router
    let router = get_router();

    // Check if the database is available
    let db = match surrealdb::SurrealClient::default() {
        Ok(db) => db,
        Err(err) => {
            return Err(AppError::DatabaseClientCreationError(format!("Failed to connect to database: {}", err)).log());
        }
    };
    match db.check_connection().await {
        Ok(_) => log::info!("Database connection established!"),
        Err(err) => {
            return Err(AppError::DatabaseConnectionError(err.to_string()).log());
        }
    }

    // Serve the app
    match Server::bind(&"0.0.0.0:8000".to_socket_addrs().unwrap().next().unwrap())
        .serve(router.into_make_service())
        .await {
            Ok(server) => server,
            Err(err) => {
                return Err(AppError::ServerStartError(err.to_string()).log());
            }
        };
    Ok(())
}