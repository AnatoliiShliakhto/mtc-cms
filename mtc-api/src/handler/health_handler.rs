use crate::error::api_error::ApiError;

pub async fn health_handler() -> Result<String, ApiError> {
    Ok("+".to_string())
}
