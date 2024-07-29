use mtc_model::list_model::RecordListModel;

use crate::error::api_error::ApiError;
use crate::handler::{ApiHandler, HandlerResponse};

pub trait PermissionsHandler {
    async fn get_permissions(&self) -> Result<RecordListModel, ApiError>;
}

impl PermissionsHandler for ApiHandler {
    async fn get_permissions(&self) -> Result<RecordListModel, ApiError> {
        self.api_client
            .get([&self.api_url, "permissions"].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }
}
