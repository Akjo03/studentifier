//! Prelude for this application.

pub use crate::error::{ApiError, AppError};

pub type Result<T> = std::result::Result<T, AppError>;
pub type ApiResult<T> = std::result::Result<T, ApiError>;