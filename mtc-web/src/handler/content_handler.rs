use mtc_model::api_model::{ApiListItemModel, ApiModel, ApiPostModel};

use crate::error::api_error::ApiError;
use crate::handler::{ApiHandler, HandlerNullResponse, HandlerResponse};

pub trait ContentHandler {
    async fn get_single_content(&self, slug: &str) -> Result<ApiModel, ApiError>;
    async fn get_collection_content(&self, schema: &str, slug: &str) -> Result<ApiModel, ApiError>;
    async fn get_content_list(&self, schema: &str) -> Result<Vec<ApiListItemModel>, ApiError>;
    async fn create_content(
        &self,
        schema: &str,
        slug: &str,
        content: &ApiPostModel,
    ) -> Result<(), ApiError>;
    async fn update_content(
        &self,
        schema: &str,
        slug: &str,
        content: &ApiPostModel,
    ) -> Result<(), ApiError>;
    async fn delete_content(&self, schema: &str, slug: &str) -> Result<(), ApiError>;
}

impl ContentHandler for ApiHandler {
    async fn get_single_content(&self, slug: &str) -> Result<ApiModel, ApiError> {
        self.api_client
            .get([&self.api_url, slug].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }

    async fn get_collection_content(&self, schema: &str, slug: &str) -> Result<ApiModel, ApiError> {
        self.api_client
            .get([&self.api_url, schema, slug].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }

    async fn get_content_list(&self, schema: &str) -> Result<Vec<ApiListItemModel>, ApiError> {
        match schema {
            "" => {
                self.api_client
                    .get([&self.api_url, "all_api"].join("/"))
                    .send()
                    .await
                    .consume_data()
                    .await
            }
            value => {
                self.api_client
                    .get([&self.api_url, value, "list", "all"].join("/"))
                    .send()
                    .await
                    .consume_data()
                    .await
            }
        }
    }

    async fn create_content(
        &self,
        schema: &str,
        slug: &str,
        content: &ApiPostModel,
    ) -> Result<(), ApiError> {
        self.api_client
            .post([&self.api_url, schema, slug].join("/"))
            .json(&content)
            .send()
            .await
            .consume()
            .await
    }

    async fn update_content(
        &self,
        schema: &str,
        slug: &str,
        content: &ApiPostModel,
    ) -> Result<(), ApiError> {
        match schema {
            "" => {
                self.api_client
                    .patch([&self.api_url, slug].join("/"))
                    .json(&content)
                    .send()
                    .await
                    .consume()
                    .await
            }
            val => {
                self.api_client
                    .patch([&self.api_url, val, slug].join("/"))
                    .json(&content)
                    .send()
                    .await
                    .consume()
                    .await
            }
        }
    }

    async fn delete_content(&self, schema: &str, slug: &str) -> Result<(), ApiError> {
        self.api_client
            .delete([&self.api_url, schema, slug].join("/"))
            .send()
            .await
            .consume()
            .await
    }
}
