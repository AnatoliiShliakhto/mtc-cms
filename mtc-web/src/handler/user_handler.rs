use mtc_model::user_model::UserModel;

use crate::error::api_error::ApiError;
use crate::handler::{ApiHandler, HandlerResponse};
use crate::model::response_model::ApiResponse;

pub trait UserHandler {
    async fn get_user_list(&self, page: usize) -> Result<ApiResponse<Vec<UserModel>>, ApiError>;
}

impl UserHandler for ApiHandler {
    async fn get_user_list(&self, page: usize) -> Result<ApiResponse<Vec<UserModel>>, ApiError> {
        self
            .api_client
            .get([&self.api_url, "user", "list", &page.to_string()].join("/"))
            .send()
            .await
            .consume_page()
            .await
    }
}