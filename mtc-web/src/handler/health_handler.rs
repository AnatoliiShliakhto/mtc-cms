use reqwest::StatusCode;

use mtc_model::HealthModel;

use crate::error::api_error::ApiError;
use crate::model::response_model::ApiResponse;
use crate::state::AppState;

pub trait HealthHandler {
    async fn health_check(&self) -> Result<String, ApiError>;
}

impl HealthHandler for AppState {
    async fn health_check(&self) -> Result<String, ApiError> {
        match self
            .api_client
            .get([&self.api_url, "health"].join("/"))
            .send()
            .await {
            Ok(response) => {
                match response.status() {
                    StatusCode::UNAUTHORIZED => Ok("anonymous".to_string()),
                    StatusCode::OK => {
                        Ok(response.json::<ApiResponse<HealthModel>>().await?.data.id)
                    }
                    _ => Err(ApiError::ResponseError("errors.health".to_string()))
                }
            }
            Err(e) => Err(ApiError::from(e))
        }
    }
}