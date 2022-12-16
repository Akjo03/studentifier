use crate::prelude::*;

use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    pub time: Option<String>,
    pub status: Option<String>,
    pub result: Option<Value>,
    pub detail: Option<String>,
}

pub struct SurrealClient {
    client: Client,
    url: SocketAddr,
    username: String,
    password: String,
    current_namespace: String,
    current_database: String,
} impl SurrealClient {
    pub fn new(url: SocketAddr, username: String, password: String) -> Self {
        Self {
            client: Client::new(),
            url,
            username,
            password,
            current_namespace: "default".to_string(),
            current_database: "default".to_string(),
        }
    }

    pub fn set_target(&mut self, namespace: String, database: String) {
        self.current_namespace = namespace;
        self.current_database = database;
    }

    pub fn default(url: SocketAddr) -> Self {
        let mut surreal_client = Self::new(url, "root".to_string(), "root".to_string());
        surreal_client.set_target(
            "akjo".to_string(), 
            "studentifier".to_string()
        );

        surreal_client
    }

    pub async fn check_connection(&self) -> Result<()> {
        let url = format!("http://{}/sql", self.url);

        match self.client.post(&url)
            .basic_auth(&self.username, Some(&self.password))
            .header("Accept", "application/json")
            .header("NS", &self.current_namespace)
            .header("DB", &self.current_database)
            .body("INFO FOR DB;")
            .send()
            .await {
                Ok(response) => {
                    let resp_object = match response.json::<Value>().await {
                        Ok(resp_array) => match resp_array.as_array() {
                            Some(resp_array) => resp_array.get(0).unwrap().clone(),
                            None => return Err(AppError::SurrealQueryError("Failed to parse response!".to_string())),
                        }
                        Err(err) => return Err(AppError::SurrealQueryError(err.to_string())),
                    };

                    let query_response = match serde_json::from_value::<QueryResponse>(resp_object) {
                        Ok(resp_object) => resp_object,
                        Err(err) => return Err(AppError::SurrealQueryError(err.to_string())),
                    };

                    match query_response.status {
                        Some(status) => {
                            if status == "OK" {
                                Ok(())
                            } else {
                                Err(AppError::SurrealQueryError(format!("Failed to execute query: {}", query_response.detail.unwrap())))
                            }
                        }
                        None => Err(AppError::SurrealQueryError("Failed to execute query!".to_string())),
                    }
                },
                Err(err) => return Err(AppError::SurrealConnectionError(err.to_string())),
            }
    }

    pub async fn sql(&self, query: String) -> Result<QueryResponse> {
        let url = format!("http://{}/sql", self.url);

        match self.client.post(&url)
            .basic_auth(&self.username, Some(&self.password))
            .header("Accept", "application/json")
            .header("NS", &self.current_namespace)
            .header("DB", &self.current_database)
            .body(query)
            .send()
            .await {
                Ok(response) => {
                    let resp_object = match response.json::<Value>().await {
                        Ok(resp_array) => match resp_array.as_array() {
                            Some(resp_array) => resp_array.get(0).unwrap().clone(),
                            None => return Err(AppError::SurrealQueryError("Failed to parse response!".to_string())),
                        }
                        Err(err) => return Err(AppError::SurrealQueryError(err.to_string())),
                    };

                    let query_response = match serde_json::from_value::<QueryResponse>(resp_object) {
                        Ok(resp_object) => resp_object,
                        Err(err) => return Err(AppError::SurrealQueryError(err.to_string())),
                    };

                    match query_response.status.clone() {
                        Some(status) => {
                            if status == "OK" {
                                Ok(query_response)
                            } else {
                                Err(AppError::SurrealQueryError(format!("Failed to execute query: {}", query_response.detail.unwrap())))
                            }
                        }
                        None => Err(AppError::SurrealQueryError("Failed to execute query!".to_string())),
                    }
                },
                Err(err) => return Err(AppError::SurrealConnectionError(err.to_string())),
            }
    }
}