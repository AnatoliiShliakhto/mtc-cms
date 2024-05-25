use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;
use tracing::warn;

use crate::error::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::pagination_model::{PaginationBuilder, PaginationModel};
use crate::model::permission_model::PermissionsModel;
use crate::model::request_model::ValidatedPayload;
use crate::model::response_model::ApiResponse;
use crate::model::role_model::{RoleCreateModel, RoleModel, RoleUpdateModel};
use crate::repository::permissions_repository::PermissionsRepositoryTrait;
use crate::repository::RepositoryPaginate;
use crate::repository::role_repository::RoleRepositoryTrait;
use crate::state::AppState;

pub async fn role_list_handler(
    page: Option<Path<usize>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<Vec<RoleModel>>> {
    session.permission("role::read").await?;

    let page: usize = match page {
        Some(Path(value)) => value,
        _ => 1
    };

    let pagination = PaginationModel::new(
        state.role_service.get_total().await?,
        state.cfg.rows_per_page,
    )
        .page(page);

    let data = state.role_service.get_page(pagination.from, pagination.per_page).await?;

    Ok(ApiResponse::DataPage(data, pagination))
}

pub async fn role_get_handler(
    Path(slug): Path<String>,
    session: Session,
    state: State<Arc<AppState>>,
) -> Result<ApiResponse<RoleModel>> {
    session.permission("role::read").await?;

    let role_model = state
        .role_service
        .find_by_slug(&slug)
        .await?;

    Ok(ApiResponse::Data(role_model))
}

pub async fn role_create_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<RoleCreateModel>,
) -> Result<ApiResponse<RoleModel>> {
    session.permission("role::write").await?;

    let role_model = state
        .role_service
        .create(payload)
        .await?;

    Ok(ApiResponse::Data(role_model))
}

pub async fn role_update_handler(
    Path(slug): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<RoleUpdateModel>,
) -> Result<ApiResponse<RoleModel>> {
    session.permission("role::write").await?;

    let role_model = state
        .role_service
        .update(&slug, payload)
        .await?;

    Ok(ApiResponse::Data(role_model))
}

pub async fn role_delete_handler(
    Path(slug): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<()>> {
    session.permission("role::delete").await?;

    state
        .role_service
        .delete(&slug)
        .await?;

    Ok(ApiResponse::Ok)
}

pub async fn role_get_permissions(
    Path(slug): Path<String>,
    session: Session,
    state: State<Arc<AppState>>,
) -> Result<ApiResponse<PermissionsModel>> {
    session.permission("role::read").await?;

    let permissions = state
        .permissions_service
        .find_by_role(&slug)
        .await?;

    Ok(ApiResponse::Data(permissions))
}

pub async fn role_set_permissions(
    Path(slug): Path<String>,
    session: Session,
    state: State<Arc<AppState>>,
    ValidatedPayload(payload): ValidatedPayload<PermissionsModel>,
) -> Result<ApiResponse<PermissionsModel>> {
    session.permission("role::write").await?;

    let role_model = state.role_service.find_by_slug(&slug).await?;

    state.role_service.permissions_drop(&role_model.id).await?;

    for permission in payload.permissions {
        match state.permissions_service.find_by_slug(&permission).await {
            Ok(value) => {
                state.role_service.permission_assign(&role_model.id, &value.id).await?
            }
            _ => warn!("can't find permission -> {permission}")
        }
    }

    let permissions = state
        .permissions_service
        .find_by_role(&slug)
        .await?;

    Ok(ApiResponse::Data(permissions))
}