use crate::prelude::*;
use crate::models::info::*;

pub async fn info() -> ApiResult<InfoResponse> {
    Ok(InfoResponse::default())
}