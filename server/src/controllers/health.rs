use crate::prelude::*;

use std::net::ToSocketAddrs;

use crate::util::surrealdb;

pub async fn health() -> ApiResult<()> {
    let db = match surrealdb::SurrealClient::default() {
        Ok(db) => db,
        Err(err) => {
            return Err(ApiError::ServerError(format!("Failed to connect to database: {}", err)).log());
        }
    };
    match db.check_connection().await {
        Ok(_) => log::info!("Database connection established!"),
        Err(err) => {
            return Err(ApiError::ServerError(format!("Failed to establish connection to database: {}", err)).log());
        }
    }

    Ok(())
}