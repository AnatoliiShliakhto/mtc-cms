pub mod api_error;
pub mod db_error;
pub mod session_error;
pub mod generic_error;

pub type Result<T> = core::result::Result<T, api_error::ApiError>;