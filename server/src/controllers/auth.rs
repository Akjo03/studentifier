use crate::prelude::*;
use crate::models::auth::*;
use crate::util::surrealdb;
use crate::util::security;

use axum::Json;

pub async fn register(request: Json<RegisterRequest>) -> ApiResult<RegisterResponse> {
    let db = match surrealdb::SurrealClient::default() {
        Ok(db) => db,
        Err(err) => {
            return Err(ApiError::ServerError(format!("Failed to connect to database: {}", err)).log());
        }
    };
    log::info!("[Auth - Register] Starting registration process...");

    // 1. Check if username is already taken
    log::info!("Checking if username is taken...");
    let username_taken = match db.sql(format!(
        "SELECT username FROM user WHERE username = '{}';", request.0.username
    )).await {
        Ok(resp) => {
            let resp_json = resp.result.unwrap();
            let resp_array = resp_json.as_array().unwrap();
            if resp_array.len() > 0 {
                true
            } else {
                false
            }
        },
        Err(_) => {
            return Err(ApiError::ServerError("Failed to check if username exists!".to_string()).log());
        }
    };

    if username_taken {
        return Err(ApiError::BadRequest("Username already taken!".to_string()).log());
    }


    // 2. Generate salt and password hash
    log::info!("Generating salt and password hash...");
    let salt = security::generate_salt();
    let password_hash = security::generate_password_hash(&request.0.password, &salt);

    // 3. Create new user
    log::info!("Creating new user...");
    let new_user = match db.sql(format!(
        "CREATE user SET username = '{}', password = '{}', salt = '{}';",
        request.0.username, password_hash, salt
    )).await {
        Ok(resp) => {
            let resp_result = resp.result.unwrap();
            let resp_json = resp_result.as_array().unwrap().get(0).unwrap();

            serde_json::from_value::<NewUser>(resp_json.clone()).unwrap()
        },
        Err(_) => {
            return Err(ApiError::ServerError("Failed to create new user!".to_string()).log());
        }
    };

    // 4. Generate access and refresh tokens
    log::info!("Generating access and refresh tokens...");
    let access_token = security::generate_access_token(&new_user.id);
    let refresh_token = security::generate_refresh_token(&new_user.id);

    // 5. Save refresh token to database
    log::info!("Saving refresh token to database...");
    match db.sql(format!(
        "UPDATE {} SET refresh_token = '{}';",
        &new_user.id, refresh_token
    )).await {
        Ok(_) => {},
        Err(_) => {
            return Err(ApiError::ServerError("Failed to save refresh token!".to_string()).log());
        }
    }

    log::info!("[Auth - Register] Registration process complete!");

    // 6. Return access and refresh tokens
    Ok(RegisterResponse::new(
        access_token,
        refresh_token,
    ))
}

pub async fn login(request: Json<LoginRequest>) -> ApiResult<LoginResponse> {
    let db = match surrealdb::SurrealClient::default() {
        Ok(db) => db,
        Err(err) => {
            return Err(ApiError::ServerError(format!("Failed to connect to database: {}", err)).log());
        }
    };
    log::info!("[Auth - Login] Starting login process...");

    // 1. Check if username exists
    log::info!("Checking if username exists...");
    match db.sql(format!(
        "SELECT username FROM user WHERE username = '{}';", request.0.username
    )).await {
        Ok(resp) => {
            let resp_json = resp.result.unwrap();
            let resp_array = resp_json.as_array().unwrap();
            if resp_array.len() == 0 {
                return Err(ApiError::BadRequest("Username does not exist!".to_string()).log());
            }
        },
        Err(_) => {
            return Err(ApiError::ServerError("Failed to check if username exists!".to_string()).log());
        }
    };

    // 2. Check if password is correct
    log::info!("Checking if password is correct...");
    // 2.1 Get salt and password hash from database
    let salty_pass = match db.sql(format!(
        "SELECT salt, password FROM user WHERE username = '{}';", request.0.username
    )).await {
        Ok(resp) => {
            let resp_json = resp.result.unwrap();
            let resp_array = resp_json.as_array().unwrap();
            let resp_object = resp_array.get(0).unwrap().as_object().unwrap();

            let salt = resp_object.get("salt").unwrap().as_str().unwrap();
            let password = resp_object.get("password").unwrap().as_str().unwrap();

            (salt.to_string(), password.to_string())
        },
        Err(_) => {
            return Err(ApiError::ServerError("Failed to get salt and password hash!".to_string()).log());
        }
    };

    // 2.2 Verify password
    if !security::verify_password_hash(&request.0.password, &salty_pass.0, &salty_pass.1) {
        return Err(ApiError::BadRequest("Password is incorrect!".to_string()).log());
    }

    // 3. Generate access and refresh tokens
    log::info!("Generating access and refresh tokens...");
    // 3.1 Get user id from database
    let user_id = match db.sql(format!(
        "SELECT id FROM user WHERE username = '{}';", request.0.username
    )).await {
        Ok(resp) => {
            let resp_json = resp.result.unwrap();
            let resp_array = resp_json.as_array().unwrap();
            let resp_object = resp_array.get(0).unwrap().as_object().unwrap();

            resp_object.get("id").unwrap().as_str().unwrap().to_string()
        },
        Err(_) => {
            return Err(ApiError::ServerError("Failed to get user id!".to_string()).log());
        }
    };

    // 3.2 Generate access and refresh tokens
    let access_token = security::generate_access_token(&user_id);
    let refresh_token = security::generate_refresh_token(&user_id);

    // 4. Save refresh token to database
    log::info!("Saving refresh token to database...");
    match db.sql(format!(
        "UPDATE {} SET refresh_token = '{}';",
        &user_id, refresh_token
    )).await {
        Ok(_) => {},
        Err(_) => {
            return Err(ApiError::ServerError("Failed to save refresh token!".to_string()).log());
        }
    }

    log::info!("[Auth - Login] Login process complete!");
    // 5. Return access and refresh tokens
    Ok(LoginResponse::new(
        access_token,
        refresh_token,
    ))
}

pub async fn refresh(request: Json<RefreshRequest>) -> ApiResult<RefreshResponse> {
    let db = match surrealdb::SurrealClient::default() {
        Ok(db) => db,
        Err(err) => {
            return Err(ApiError::ServerError(format!("Failed to connect to database: {}", err)).log());
        }
    };
    log::info!("[Auth - Refresh] Starting refresh process...");

    // 1. Check if refresh token exists
    log::info!("Checking if refresh token exists...");
    // 1.1 Get user id from refresh token
    let user_id = match db.sql(format!(
        "SELECT id FROM user WHERE refresh_token = '{}';", 
        request.0.refresh_token
    )).await {
        Ok(resp) => {
            let resp_json = resp.result.unwrap();
            let resp_array = resp_json.as_array().unwrap();
            if resp_array.len() == 0 {
                return Err(ApiError::BadRequest("Refresh token does not exist!".to_string()).log());
            }

            let resp_object = resp_array.get(0).unwrap().as_object().unwrap();
            resp_object.get("id").unwrap().as_str().unwrap().to_string()
        },
        Err(_) => {
            return Err(ApiError::ServerError("Failed to check if refresh token exists!".to_string()).log());
        }
    };

    // 2. Generate new access and refresh tokens
    log::info!("Generating new access and refresh tokens...");
    let access_token = security::generate_access_token(&user_id);
    let refresh_token = security::generate_refresh_token(&user_id);

    // 3. Save new refresh token to database
    log::info!("Saving new refresh token to database...");
    match db.sql(format!(
        "UPDATE {} SET refresh_token = '{}';",
        &user_id, refresh_token
    )).await {
        Ok(_) => {},
        Err(_) => {
            return Err(ApiError::ServerError("Failed to save new refresh token!".to_string()).log());
        }
    }

    log::info!("[Auth - Refresh] Refresh process complete!");
    // 4. Return new access and refresh tokens
    Ok(RefreshResponse::new(
        access_token,
        refresh_token,
    ))
}

pub async fn logout(request: Json<LogoutRequest>) -> ApiResult<LogoutResponse> {
    let db = match surrealdb::SurrealClient::default() {
        Ok(db) => db,
        Err(err) => {
            return Err(ApiError::ServerError(format!("Failed to connect to database: {}", err)).log());
        }
    };
    log::info!("[Auth - Logout] Starting logout process...");

    // 1. Check if refresh token exists
    log::info!("Checking if refresh token exists...");
    // 1.1 Get user id from refresh token
    let user_id = match db.sql(format!(
        "SELECT id FROM user WHERE refresh_token = '{}';", 
        request.0.refresh_token
    )).await {
        Ok(resp) => {
            let resp_json = resp.result.unwrap();
            let resp_array = resp_json.as_array().unwrap();
            if resp_array.len() == 0 {
                return Err(ApiError::BadRequest("Refresh token does not exist!".to_string()).log());
            }

            let resp_object = resp_array.get(0).unwrap().as_object().unwrap();
            resp_object.get("id").unwrap().as_str().unwrap().to_string()
        },
        Err(_) => {
            return Err(ApiError::ServerError("Failed to check if refresh token exists!".to_string()).log());
        }
    };

    // 2. Delete refresh token from database
    log::info!("Deleting refresh token from database...");
    match db.sql(format!(
        "UPDATE {} SET refresh_token = NULL;", &user_id
    )).await {
        Ok(_) => {},
        Err(_) => {
            return Err(ApiError::ServerError("Failed to delete refresh token!".to_string()).log());
        }
    }

    log::info!("[Auth - Logout] Logout process complete!");
    // 3. Return success
    Ok(LogoutResponse::new("Success".to_string()))
}
