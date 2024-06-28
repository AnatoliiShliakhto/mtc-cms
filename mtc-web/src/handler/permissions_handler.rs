use mtc_model::permission_model::PermissionsModel;

use crate::error::api_error::ApiError;
use crate::handler::{ApiHandler, HandlerResponse};

pub trait PermissionsHandler {
    async fn get_permissions(&self) -> Result<PermissionsModel, ApiError>;
}

impl PermissionsHandler for ApiHandler {
    async fn get_permissions(&self) -> Result<PermissionsModel, ApiError> {
        self
            .api_client
            .get([&self.api_url, "permissions"].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }
}