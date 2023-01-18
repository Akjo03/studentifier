use crate::prelude::*;
use crate::models::persons::*;
use crate::util::surrealdb;
use crate::services::auth::AuthService;
use axum_auth::AuthBearer;

pub async fn get_all_persons(AuthBearer(token): AuthBearer) -> ApiResult<AllPersonResponse> {
    let claims = AuthService::get_claims(token);

    let db = match surrealdb::SurrealClient::default() {
        Ok(db) => db,
        Err(err) => {
            return Err(ApiError::ServerError(format!("Failed to connect to database: {}", err)).log());
        }
    };
    log::info!("[Persons - Get All Persons] Retrieving all persons...");

    let sql_select = match claims.get("role").unwrap().as_str() {
        "admin" => format!("SELECT * FROM person"),
        "user1" => format!("SELECT * FROM person"),
        "user2" => format!("SELECT * FROM person"),
        _ => format!("SELECT * FROM person")
    };

    let persons = match db.sql(sql_select).await {
        Ok(resp) => {
            let resp_result = resp.result.unwrap();
            let person_list = serde_json::from_value::<Vec<Person>>(resp_result.clone()).unwrap();
            AllPersonResponse::new(person_list)
        },
        Err(err) => {
            return Err(ApiError::ServerError(format!("Failed to retrieve all persons: {}", err)).log());
        }
    };

    Ok(persons)
}