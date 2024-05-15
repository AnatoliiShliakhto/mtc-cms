use crate::error::Result;
use crate::model::response_model::ApiResponse;

pub async fn health_handler() -> Result<ApiResponse<()>> {
    Ok(ApiResponse::Ok)
}
