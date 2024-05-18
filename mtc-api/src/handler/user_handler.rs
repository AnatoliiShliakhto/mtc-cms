use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;

use crate::error::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::pagination_model::{PaginationBuilder, PaginationModel};
use crate::model::request_model::{PageRequest, ValidatedPayload};
use crate::model::response_model::ApiResponse;
use crate::model::user_model::{UserAssignRolesModel, UserCreateModel, UserModel, UserUpdateModel};
use crate::repository::group_repository::GroupRepositoryTrait;
use crate::repository::role_repository::RoleRepositoryTrait;
use crate::repository::user_repository::UserRepositoryTrait;
use crate::repository::RepositoryPaginate;
use crate::state::AppState;

pub async fn user_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<PageRequest>,
) -> Result<ApiResponse<Vec<UserModel>>> {
    session.permission("users::read").await?;

    let pagination = PaginationModel::new(
        state.user_service.get_total().await?,
        state.cfg.rows_per_page,
    )
        .page(payload.page.unwrap_or(1));

    let data = state.user_service.get_page(pagination.from, pagination.per_page).await?;

    Ok(ApiResponse::DataPage(data, pagination))
}

pub async fn user_get_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<UserModel>> {
    session.permission("users::read").await?;

    let user_model = state
        .user_service
        .find(&id)
        .await?;

    Ok(ApiResponse::Data(user_model))
}

pub async fn user_create_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<UserCreateModel>,
) -> Result<ApiResponse<UserModel>> {
    session.permission("users::write").await?;

    let user_model = state
        .user_service
        .create(payload)
        .await?;

    Ok(ApiResponse::Data(user_model))
}

pub async fn user_update_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<UserUpdateModel>,
) -> Result<ApiResponse<UserModel>> {
    session.permission("users::write").await?;

    let user_model = state
        .user_service
        .update(&id, payload)
        .await?;

    Ok(ApiResponse::Data(user_model))
}

pub async fn user_delete_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<()>> {
    session.permission("users::delete").await?;

    state
        .user_service
        .delete(&id)
        .await?;

    Ok(ApiResponse::Ok)
}


pub async fn user_roles_assign_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<UserAssignRolesModel>,
) -> Result<ApiResponse<()>> {
    session.permission("users::write").await?;

    let user_model = state.user_service.find(&id).await?;

    for role_id in payload.roles.iter() {
        let role_model = state.role_service.find(role_id).await?;
        state.user_service.role_assign(&user_model.id, &role_model.id).await?;
    }

    Ok(ApiResponse::Ok)
}

pub async fn user_role_assign_handler(
    Path((id, role_id)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<()>> {
    session.permission("users::write").await?;

    let role_model = state.role_service.find(&role_id).await?;
    let user_model = state.user_service.find(&id).await?;
    state.user_service.role_assign(&user_model.id, &role_model.id).await?;

    Ok(ApiResponse::Ok)
}

pub async fn user_role_unassign_handler(
    Path((id, role_id)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<()>> {
    session.permission("users::write").await?;

    let user_model = state.user_service.find(&id).await?;
    let role_model = state.role_service.find(&role_id).await?;
    state.user_service.role_unassign(&user_model.id, &role_model.id).await?;

    Ok(ApiResponse::Ok)
}

pub async fn user_group_assign_handler(
    Path((id, group_id)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<()>> {
    session.permission("users::write").await?;

    let group_model = state.group_service.find(&group_id).await?;
    let user_model = state.user_service.find(&id).await?;
    state.user_service.group_assign(&user_model.id, &group_model.id).await?;

    Ok(ApiResponse::Ok)
}

pub async fn user_group_unassign_handler(
    Path((id, group_id)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<()>> {
    session.permission("users::write").await?;

    let user_model = state.user_service.find(&id).await?;
    let group_model = state.group_service.find(&group_id).await?;
    state.user_service.group_unassign(&user_model.id, &group_model.id).await?;

    Ok(ApiResponse::Ok)
}