use crate::prelude::*;

use std::net::SocketAddr;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    pub time: Option<String>,
    pub status: Option<String>,
    pub result: Option<Value>,
    pub details: Option<String>,
}

pub struct SurrealClient {
    client: Client,
    url: SocketAddr,
    username: String,
    password: String,
    current_ns: String,
    current_db: String,
} impl SurrealClient {
    pub fn new(url: SocketAddr, username: String, password: String) -> Self {
        Self {
            client: Client::new(),
            url,
            username,
            password,
            current_ns: "default".to_string(),
            current_db: "default".to_string(),
        }
    }

    pub fn default(url: SocketAddr) -> Self {
        let mut db = Self::new(url, "root".to_string(), "root".to_string());
        db.set_target(
            "akjo".to_string(), 
            "studentifier".to_string()
        );

        db
    }

    pub fn set_target(&mut self, ns: String, db: String) {
        self.current_ns = ns;
        self.current_db = db;
    }

    pub async fn sql(&self, query: String) -> Result<QueryResponse> {
        match self.client.post(format!("http://{}/sql", self.url))
            .basic_auth(&self.username, Some(&self.password))
            .header("Accept", "application/json")
            .header("NS", &self.current_ns)
            .header("DB", &self.current_db)
            .body(query)
            .send()
            .await {
                Ok(resp) => {
                    match resp.json::<Value>().await {
                        Ok(resp) => {
                            let resp_object = match resp.as_array() {
                                Some(array) => array[0].clone(),
                                None => resp
                            }.as_object().unwrap().clone();
                            let json_resp = serde_json::from_value::<QueryResponse>(resp_object.clone().into())
                                .map_err(|err| AppError::QueryResponseParseError(err.to_string()).log());
                            if json_resp.is_err() {
                                return Err(json_resp.err().unwrap());
                            }
                            let json_resp = json_resp.unwrap();
                            if json_resp.status == Some("OK".to_string()) {
                                Ok(json_resp)
                            } else {
                                Err(AppError::QueryExecutionError(json_resp.details.unwrap_or("Unknown error!".to_string())).log())
                            }
                        },
                        Err(err) => Err(AppError::QueryResponseParseError(err.to_string()).log())
                    }
                },
                Err(err) => Err(AppError::QuerySendError(err.to_string()).log())
            }
    }

    pub async fn check_connection(&self) -> Result<()> {
        match self.sql("INFO FOR DB;".to_string()).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }
}