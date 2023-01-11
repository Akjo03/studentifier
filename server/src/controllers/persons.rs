use crate::prelude::*;
use crate::models::persons::*;

pub async fn get_all_persons() -> ApiResult<AllPersonResponse> {
    Ok(AllPersonResponse::new(Vec::new()))
}