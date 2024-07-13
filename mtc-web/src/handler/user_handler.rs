use mtc_model::group_model::GroupsModel;
use mtc_model::role_model::RolesModel;
use mtc_model::user_model::{UserCreateModel, UserModel, UsersModel, UserUpdateModel};

use crate::error::api_error::ApiError;
use crate::handler::{ApiHandler, HandlerNullResponse, HandlerResponse};
use crate::model::response_model::ApiResponse;

pub trait UserHandler {
    async fn get_user_list(&self, page: usize) -> Result<ApiResponse<Vec<UserModel>>, ApiError>;
    async fn get_user_groups(&self, login: &str) -> Result<GroupsModel, ApiError>;
    async fn get_user_roles(&self, login: &str) -> Result<RolesModel, ApiError>;
    async fn delete_user(&self, login: &str) -> Result<(), ApiError>;
    async fn delete_user_list(&self, users: UsersModel) -> Result<(), ApiError>;
    async fn create_user(&self, login: &str, user: &UserCreateModel)
        -> Result<UserModel, ApiError>;
    async fn update_user(&self, login: &str, user: &UserUpdateModel)
        -> Result<UserModel, ApiError>;
    async fn toggle_block_user(&self, login: &str) -> Result<(), ApiError>;
}

impl UserHandler for ApiHandler {
    async fn get_user_list(&self, page: usize) -> Result<ApiResponse<Vec<UserModel>>, ApiError> {
        self.api_client
            .get([&self.api_url, "user", "list", &page.to_string()].join("/"))
            .send()
            .await
            .consume_page()
            .await
    }

    async fn get_user_groups(&self, login: &str) -> Result<GroupsModel, ApiError> {
        self
            .api_client
            .get([&self.api_url, "user", login, "groups"].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }

    async fn get_user_roles(&self, login: &str) -> Result<RolesModel, ApiError> {
        self
            .api_client
            .get([&self.api_url, "user", login, "roles"].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }

    async fn delete_user(&self, login: &str) -> Result<(), ApiError> {
        self.api_client
            .delete([&self.api_url, "user", login].join("/"))
            .send()
            .await
            .consume()
            .await
    }

    async fn delete_user_list(&self, users: UsersModel) -> Result<(), ApiError> {
        self.api_client
            .delete([&self.api_url, "user", "list"].join("/"))
            .json(&users)
            .send()
            .await
            .consume()
            .await
    }

    async fn create_user(
        &self,
        login: &str,
        user: &UserCreateModel,
    ) -> Result<UserModel, ApiError> {
        self.api_client
            .post([&self.api_url, "user", login].join("/"))
            .json(user)
            .send()
            .await
            .consume_data()
            .await
    }

    async fn update_user(
        &self,
        login: &str,
        user: &UserUpdateModel,
    ) -> Result<UserModel, ApiError> {
        self.api_client
            .patch([&self.api_url, "user", login].join("/"))
            .json(user)
            .send()
            .await
            .consume_data()
            .await
    }

    async fn toggle_block_user(&self, login: &str) -> Result<(), ApiError> {
        self.api_client
            .post([&self.api_url, "user", login, "block"].join("/"))
            .send()
            .await
            .consume()
            .await
    }
}
