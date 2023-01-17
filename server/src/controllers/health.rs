use crate::prelude::*;

use std::net::ToSocketAddrs;

use crate::util::surrealdb;

pub async fn health() -> ApiResult<()> {
    let deploy_mode = match std::env::var("DEPLOY") {
        Ok(val) => val,
        Err(_) => "false".to_string(),
    };
    let connection_str = match deploy_mode.as_str() {
        "render" => "studentifier-database.onrender.com:8000",
        "docker" => "database:8000",
        _ => "127.0.0.1:8000",
    };

    let db = surrealdb::SurrealClient::default(match connection_str.to_socket_addrs() {
        Ok(mut addr) => addr.next().unwrap_or(([127, 0, 0, 1], 8000).into()),
        Err(err) => {
            return Err(ApiError::ServerError(format!("Failed to establish connection to database: {}", err)).log());
        }
    });
    match db.check_connection().await {
        Ok(_) => log::info!("Database connection established!"),
        Err(err) => {
            return Err(ApiError::ServerError(format!("Failed to establish connection to database: {}", err)).log());
        }
    }

    Ok(())
}