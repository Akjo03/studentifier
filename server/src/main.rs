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
    let addr = match match std::env::var("DEPLOY") {
        Ok(deploy_mode) => {
            log::info!("Running on deploy mode: {}", deploy_mode);
            match deploy_mode.as_str() {
                "render" | "docker" => "0.0.0.0:3000".to_string(),
                _ => "127.0.0.1:3000".to_string()
            }
        },
        Err(_) => "127.0.0.1:3000".to_string()
    }.to_socket_addrs() {
        Ok(mut addr) => addr.next().unwrap_or(([127, 0, 0, 1], 3000).into()),
        Err(err) => {
            return Err(AppError::ServerStartError(err.to_string()).log());
        }
    };
    log::info!("Server listening on {}...", addr);
    match Server::bind(&addr)
        .serve(router.into_make_service())
        .await {
            Ok(server) => server,
            Err(err) => {
                return Err(AppError::ServerStartError(err.to_string()).log());
            }
        };

    Ok(())
}