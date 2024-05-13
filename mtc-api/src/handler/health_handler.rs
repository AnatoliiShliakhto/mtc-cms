use crate::error::api_error::ApiError;
use crate::model::response_model::ApiResponse;

pub async fn health_handler() -> Result<ApiResponse<()>, ApiError> {
    Ok(ApiResponse::Ok)
}
