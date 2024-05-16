use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;

use crate::error::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::request_model::{PageRequest, ValidatedPayload};
use crate::model::response_model::ApiResponse;
use crate::model::role_model::{RoleCreateModel, RoleModel, RoleUpdateModel};
use crate::paginator::{ModelPagination, ServicePaginate};
use crate::service::permissions_service::PermissionsServiceTrait;
use crate::service::role_service::RoleServiceTrait;
use crate::state::AppState;

pub async fn role_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<PageRequest>,
) -> Result<ApiResponse<ModelPagination<Vec<RoleModel>>>> {
    session.permission("roles::read").await?;

    let role_pagination = state.role_service
        .paginate(payload.page.unwrap_or(1)).await?;

    Ok(ApiResponse::Json(role_pagination))
}

pub async fn role_get_handler(
    Path(id): Path<String>,
    session: Session,
    state: State<Arc<AppState>>,
) -> Result<ApiResponse<RoleModel>> {
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
    ValidatedPayload(payload): ValidatedPayload<RoleCreateModel>,
) -> Result<ApiResponse<RoleModel>> {
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
    ValidatedPayload(payload): ValidatedPayload<RoleUpdateModel>,
) -> Result<ApiResponse<RoleModel>> {
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
) -> Result<ApiResponse<()>> {
    session.permission("roles::delete").await?;

    state
        .role_service
        .delete(&id)
        .await?;

    Ok(ApiResponse::Ok)
}

pub async fn role_permission_assign_handler(
    Path((id, permission_id)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<()>> {
    session.permission("roles::write").await?;

    let role_model = state.role_service.find(&id).await?;
    let permission_model = state.permissions_service.find(&permission_id).await?;
    state.role_service.permission_assign(&role_model.id, &permission_model.id).await?;

    Ok(ApiResponse::Ok)
}

pub async fn role_permission_unassign_handler(
    Path((id, permission_id)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<()>> {
    session.permission("roles::write").await?;

    let role_model = state.role_service.find(&id).await?;
    let permission_model = state.permissions_service.find(&permission_id).await?;
    state.role_service.permission_unassign(&role_model.id, &permission_model.id).await?;

    Ok(ApiResponse::Ok)
}