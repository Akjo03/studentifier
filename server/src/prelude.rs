pub use crate::error::*;

pub type Result<T> = std::result::Result<T, AppError>;
pub type ApiResult<T> = std::result::Result<T, ApiError>;