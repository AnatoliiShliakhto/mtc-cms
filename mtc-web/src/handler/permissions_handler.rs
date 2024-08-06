use mtc_model::list_model::RecordListModel;
use mtc_model::permission_model::{PermissionDtoModel, PermissionModel};

use crate::error::api_error::ApiError;
use crate::handler::{ApiHandler, HandlerNullResponse, HandlerResponse};

pub trait PermissionsHandler {
    async fn get_permissions(&self) -> Result<RecordListModel, ApiError>;
    async fn get_custom_permissions(&self) -> Result<Vec<PermissionModel>, ApiError>;
    async fn create_custom_permission(&self, permission: &PermissionDtoModel) -> Result<(), ApiError>;
    async fn delete_custom_permission(&self, permission: &PermissionDtoModel) -> Result<(), ApiError>;
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

    async fn get_custom_permissions(&self) -> Result<Vec<PermissionModel>, ApiError> {
        self.api_client
            .get([&self.api_url, "permissions", "custom"].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }

    async fn create_custom_permission(&self, permission: &PermissionDtoModel) -> Result<(), ApiError> {
        self.api_client
            .post([&self.api_url, "permissions", "custom"].join("/"))
            .json(permission)
            .send()
            .await
            .consume()
            .await
    }

    async fn delete_custom_permission(&self, permission: &PermissionDtoModel) -> Result<(), ApiError> {
        self.api_client
            .delete([&self.api_url, "permissions", "custom"].join("/"))
            .json(permission)
            .send()
            .await
            .consume()
            .await
    }
}
