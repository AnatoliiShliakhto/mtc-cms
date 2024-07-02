use mtc_model::group_model::{GroupCreateModel, GroupModel, GroupsModel, GroupUpdateModel};
use crate::error::api_error::ApiError;
use crate::handler::{ApiHandler, HandlerResponse};
use crate::model::response_model::ApiResponse;

pub trait GroupHandler {
    async fn get_group_all(&self) -> Result<GroupsModel, ApiError>;
    async fn get_group_list(&self, page: usize) -> Result<ApiResponse<Vec<GroupModel>>, ApiError>;
    async fn delete_group(&self, slug: &str) -> Result<(), ApiError>;
    async fn delete_group_list(&self, groups: GroupsModel) -> Result<(), ApiError>;
    async fn create_group(&self, slug: &str, group: &GroupCreateModel) -> Result<GroupModel, ApiError>;
    async fn update_group(&self, slug: &str, group: &GroupUpdateModel) -> Result<GroupModel, ApiError>;
}

impl GroupHandler for ApiHandler {
    async fn get_group_all(&self) -> Result<GroupsModel, ApiError> {
        self
            .api_client
            .get([&self.api_url, "group", "all"].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }
    
    async fn get_group_list(&self, page: usize) -> Result<ApiResponse<Vec<GroupModel>>, ApiError> {
        self
            .api_client
            .get([&self.api_url, "group", "list", &page.to_string()].join("/"))
            .send()
            .await
            .consume_page()
            .await
    }

    async fn delete_group(&self, slug: &str) -> Result<(), ApiError> {
        self
            .api_client
            .delete([&self.api_url, "group", slug].join("/"))
            .send()
            .await?;
        Ok(())
    }

    async fn delete_group_list(&self, groups: GroupsModel) -> Result<(), ApiError> {
        self
            .api_client
            .delete([&self.api_url, "group", "list"].join("/"))
            .json(&groups)
            .send()
            .await?;
        Ok(())
    }

    async fn create_group(&self, slug: &str, group: &GroupCreateModel) -> Result<GroupModel, ApiError> {
        self
            .api_client
            .post([&self.api_url, "group", slug].join("/"))
            .json(group)
            .send()
            .await
            .consume_data()
            .await
    }

    async fn update_group(&self, slug: &str, group: &GroupUpdateModel) -> Result<GroupModel, ApiError> {
        self
            .api_client
            .patch([&self.api_url, "group", slug].join("/"))
            .json(group)
            .send()
            .await
            .consume_data()
            .await
    }
}