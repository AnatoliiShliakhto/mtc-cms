use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;

use crate::error::api_error::ApiError;
use crate::middleware::auth_middleware::UserSession;
use crate::model::group_model::{GroupCreateModel, GroupModel, GroupUpdateModel};
use crate::model::request_model::{ApiJson, PageRequest};
use crate::model::response_model::ApiResponse;
use crate::paginator::{ModelPagination, ServicePaginate};
use crate::service::group_service::GroupServiceTrait;
use crate::state::AppState;

pub async fn group_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ApiJson(payload): ApiJson<PageRequest>,
) -> Result<ApiResponse<ModelPagination<Vec<GroupModel>>>, ApiError> {
    session.permission("groups::read").await?;

    let group_pagination = state.group_service
        .paginate(payload.page.unwrap_or(1)).await?;

    Ok(ApiResponse::Json(group_pagination))
}

pub async fn group_get_handler(
    Path(id): Path<String>,
    session: Session,
    state: State<Arc<AppState>>,
) -> Result<ApiResponse<GroupModel>, ApiError> {
    session.permission("groups::read").await?;

    let group_model = state
        .group_service
        .find(&id)
        .await?;

    Ok(ApiResponse::Data(group_model))
}

pub async fn group_create_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ApiJson(payload): ApiJson<GroupCreateModel>,
) -> Result<ApiResponse<GroupModel>, ApiError> {
    session.permission("groups::write").await?;

    let group_model = state
        .group_service
        .create(payload)
        .await?;

    Ok(ApiResponse::Data(group_model))
}

pub async fn group_update_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ApiJson(payload): ApiJson<GroupUpdateModel>,
) -> Result<ApiResponse<GroupModel>, ApiError> {
    session.permission("groups::write").await?;

    let group_model = state
        .group_service
        .update(&id, payload)
        .await?;

    Ok(ApiResponse::Data(group_model))
}

pub async fn group_delete_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<()>, ApiError> {
    session.permission("groups::delete").await?;

    state
        .group_service
        .delete(&id)
        .await?;

    Ok(ApiResponse::Ok)
}