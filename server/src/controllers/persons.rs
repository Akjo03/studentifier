use crate::prelude::*;
use crate::models::persons::*;
use crate::util::surrealdb;

pub async fn get_all_persons() -> ApiResult<AllPersonResponse> {
    let db = match surrealdb::SurrealClient::default() {
        Ok(db) => db,
        Err(err) => {
            return Err(ApiError::ServerError(format!("Failed to connect to database: {}", err)).log());
        }
    };
    log::info!("[Persons - Get All Persons] Retrieving all persons...");

    let persons = match db.sql(format!(
        "SELECT * FROM person"
    )).await {
        Ok(resp) => {
            let resp_result = resp.result.unwrap();
        },
        Err(err) => {
            return Err(ApiError::ServerError(format!("Failed to retrieve all persons: {}", err)).log());
        }
    };

    Ok(persons)
}