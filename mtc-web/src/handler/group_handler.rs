use mtc_model::group_model::GroupModel;
use crate::error::api_error::ApiError;
use crate::handler::HandlerResponse;
use crate::model::response_model::ApiResponse;
use crate::state::AppState;

pub trait GroupHandler {
    async fn get_group_list(&self, page: usize) -> Result<ApiResponse<Vec<GroupModel>>, ApiError>;
}

impl GroupHandler for AppState {
    async fn get_group_list(&self, page: usize) -> Result<ApiResponse<Vec<GroupModel>>, ApiError> {
        self
            .api_client
            .get([&self.api_url, "group", "list", &page.to_string()].join("/"))
            .send()
            .await
            .consume_page()
            .await
    }
}