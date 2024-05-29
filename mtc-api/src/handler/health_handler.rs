use crate::handler::Result;
use crate::model::response_model::ApiResponse;

pub async fn health_handler() -> Result<()> {
    Ok(ApiResponse::Ok)
}
