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
    let deploy_mode = match std::env::var("DEPLOY") {
        Ok(val) => val,
        Err(_) => "false".to_string(),
    };
    let connection_str = match deploy_mode.as_str() {
        "render" => "studentifier-database.onrender.com:8000",
        "docker" => "database:8000",
        _ => "127.0.0.1:8000",
    };

    log::info!("Connecting to database at {}...", connection_str);

    let db = surrealdb::SurrealClient::default(match connection_str.to_socket_addrs() {
        Ok(mut addr) => addr.next().unwrap_or(([127, 0, 0, 1], 8000).into()),
        Err(err) => {
            return Err(AppError::DatabaseInvalidConnectionError(err.to_string()).log());
        }
    });
    match db.check_connection().await {
        Ok(_) => log::info!("Database connection established!"),
        Err(err) => {
            return Err(AppError::DatabaseConnectionError(err.to_string()).log());
        }
    }

    // Serve the app
    let addr = ([127, 0, 0, 1], 3000).into();
    log::info!("Server listening on http://{}/...", addr);
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