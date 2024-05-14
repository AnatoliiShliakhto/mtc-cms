use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;

use crate::error::api_error::ApiError;
use crate::middleware::auth_middleware::UserSession;
use crate::model::request_model::{ApiJson, PageRequest};
use crate::model::response_model::ApiResponse;
use crate::model::role_model::{RoleCreateModel, RoleModel, RoleUpdateModel};
use crate::paginator::{ModelPagination, ServicePaginate};
use crate::service::role_service::RoleServiceTrait;
use crate::state::AppState;

pub async fn role_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ApiJson(payload): ApiJson<PageRequest>,
) -> Result<ApiResponse<ModelPagination<Vec<RoleModel>>>, ApiError> {
    session.permission("roles::read").await?;

    let role_pagination = state.role_service
        .paginate(payload.page.unwrap_or(1)).await?;

    Ok(ApiResponse::Json(role_pagination))
}

pub async fn role_get_handler(
    Path(id): Path<String>,
    session: Session,
    state: State<Arc<AppState>>,
) -> Result<ApiResponse<RoleModel>, ApiError> {
    session.permission("roles::read").await?;

    let role_model = state
        .role_service
        .find(&id)
        .await?;

    Ok(ApiResponse::Data(role_model))
}

pub async fn role_create_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ApiJson(payload): ApiJson<RoleCreateModel>,
) -> Result<ApiResponse<RoleModel>, ApiError> {
    session.permission("roles::write").await?;

    let role_model = state
        .role_service
        .create(payload)
        .await?;

    Ok(ApiResponse::Data(role_model))
}

pub async fn role_update_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ApiJson(payload): ApiJson<RoleUpdateModel>,
) -> Result<ApiResponse<RoleModel>, ApiError> {
    session.permission("roles::write").await?;

    let role_model = state
        .role_service
        .update(&id, payload)
        .await?;

    Ok(ApiResponse::Data(role_model))
}

pub async fn role_delete_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<()>, ApiError> {
    session.permission("roles::delete").await?;

    state
        .role_service
        .delete(&id)
        .await?;

    Ok(ApiResponse::Ok)
}