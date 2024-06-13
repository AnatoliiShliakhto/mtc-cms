use mtc_model::HealthModel;

use crate::error::api_error::ApiError;
use crate::handler::HandlerResponse;
use crate::state::AppState;

pub trait HealthHandler {
    async fn health_check(&self) -> Result<HealthModel, ApiError>;
}

impl HealthHandler for AppState {
    async fn health_check(&self) -> Result<HealthModel, ApiError> {
        self
            .api_client
            .get([&self.api_url, "health"].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }
}