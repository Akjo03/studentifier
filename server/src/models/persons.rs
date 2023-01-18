use axum::{response::IntoResponse, Json};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    id: String,
    person_id: String,
    first_name: String,
    last_name: String,
    street: String,
    city: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllPersonResponse {
    persons: Vec<Person>,
} impl IntoResponse for AllPersonResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
} impl AllPersonResponse {
    pub fn new(persons: Vec<Person>) -> Self {
        Self {
            persons,
        }
    }
}