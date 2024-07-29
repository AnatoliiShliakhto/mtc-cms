use mtc_model::storage_model::StoragesModel;

use crate::error::api_error::ApiError;
use crate::handler::{ApiHandler, HandlerNullResponse, HandlerResponse};

pub trait StorageHandler {
    async fn get_storage_files(&self, path: &str) -> Result<StoragesModel, ApiError>;
    async fn delete_file(&self, api_id: &str, file_name: &str) -> Result<(), ApiError>;
}

impl StorageHandler for ApiHandler {
    async fn get_storage_files(&self, path: &str) -> Result<StoragesModel, ApiError> {
        self.api_client
            .get([&self.api_url, path].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }

    async fn delete_file(&self, api: &str, file_name: &str) -> Result<(), ApiError> {
        self.api_client
            .delete([&self.api_url, api, file_name].join("/"))
            .send()
            .await
            .consume()
            .await
    }
}
