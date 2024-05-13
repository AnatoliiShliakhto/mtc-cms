use std::sync::Arc;

use axum::extract::{Path, State};

use crate::error::api_error::ApiError;
use crate::model::request_model::{ApiJson, PageRequest};
use crate::model::response_model::ApiResponse;
use crate::model::user_model::UserModel;
use crate::paginator::ModelPagination;
use crate::paginator::ServicePaginate;
use crate::service::user_service::UserServiceTrait;
use crate::state::AppState;

pub async fn user_list_handler(
    state: State<Arc<AppState>>,
    ApiJson(query_param): ApiJson<PageRequest>,
) -> Result<ApiResponse<ModelPagination<Vec<UserModel>>>, ApiError> {
    let user_pagination = state.user_service
        .paginate(query_param.page.unwrap_or(1)).await?;

    Ok(ApiResponse::Json(user_pagination))
}

pub async fn user_get_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
) -> Result<ApiResponse<UserModel>, ApiError> {
    let user_model = state
        .user_service
        .find(id)
        .await?;

    Ok(ApiResponse::Data(user_model))
}