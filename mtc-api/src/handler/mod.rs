pub mod health_handler;
pub mod role_handler;
pub mod setup_handler;
pub mod user_handler;
pub mod permissions_handler;
pub mod auth_handler;
pub mod group_handler;
pub mod schema_handler;
pub mod api_handler;
pub mod store_handler;

pub type Result<T> =
core::result::Result<
    crate::model::response_model::ApiResponse<T>,
    crate::error::api_error::ApiError
>;