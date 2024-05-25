use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;
use tracing::warn;

use crate::error::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::group_model::GroupsModel;
use crate::model::pagination_model::{PaginationBuilder, PaginationModel};
use crate::model::permission_model::PermissionsModel;
use crate::model::request_model::ValidatedPayload;
use crate::model::response_model::ApiResponse;
use crate::model::role_model::RolesModel;
use crate::model::user_model::{UserCreateModel, UserModel, UserUpdateModel};
use crate::repository::group_repository::GroupRepositoryTrait;
use crate::repository::permissions_repository::PermissionsRepositoryTrait;
use crate::repository::RepositoryPaginate;
use crate::repository::role_repository::RoleRepositoryTrait;
use crate::repository::user_repository::UserRepositoryTrait;
use crate::state::AppState;

pub async fn user_list_handler(
    page: Option<Path<usize>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<Vec<UserModel>>> {
    session.permission("user::read").await?;

    let page: usize = match page {
        Some(Path(value)) => value,
        _ => 1
    };

    let pagination = PaginationModel::new(
        state.user_service.get_total().await?,
        state.cfg.rows_per_page,
    )
        .page(page);

    let data = state.user_service.get_page(pagination.from, pagination.per_page).await?;

    Ok(ApiResponse::DataPage(data, pagination))
}

pub async fn user_get_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<UserModel>> {
    session.permission("user::read").await?;

    let user_model = state
        .user_service
        .find_by_login(&login)
        .await?;

    Ok(ApiResponse::Data(user_model))
}

pub async fn user_create_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<UserCreateModel>,
) -> Result<ApiResponse<UserModel>> {
    session.permission("user::write").await?;

    let user_model = state
        .user_service
        .create(payload)
        .await?;

    Ok(ApiResponse::Data(user_model))
}

pub async fn user_update_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<UserUpdateModel>,
) -> Result<ApiResponse<UserModel>> {
    session.permission("user::write").await?;

    let user_model = state
        .user_service
        .update(&login, payload)
        .await?;

    Ok(ApiResponse::Data(user_model))
}

pub async fn user_delete_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<()>> {
    session.permission("user::delete").await?;

    state
        .user_service
        .delete(&login)
        .await?;

    Ok(ApiResponse::Ok)
}

pub async fn user_get_roles_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<RolesModel>> {
    session.permission("user::read").await?;

    let roles = state
        .role_service
        .find_by_user(&login)
        .await?;

    Ok(ApiResponse::Data(roles))
}

pub async fn user_set_roles_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<RolesModel>,
) -> Result<ApiResponse<RolesModel>> {
    session.permission("user::write").await?;

    let user_model = state.user_service.find_by_login(&login).await?;

    state.user_service.roles_drop(&user_model.id).await?;

    for role in payload.roles {
        match state.role_service.find_by_slug(&role).await {
            Ok(value) => {
                state.user_service.role_assign(&user_model.id, &value.id).await?
            }
            _ => warn!("can't find role -> {role}")
        }
    }

    let roles = state
        .role_service
        .find_by_user(&login)
        .await?;

    Ok(ApiResponse::Data(roles))
}

pub async fn user_get_permissions_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<PermissionsModel>> {
    session.permission("user::read").await?;

    let permissions = state
        .permissions_service
        .find_by_user(&login)
        .await?;

    Ok(ApiResponse::Data(permissions))
}

pub async fn user_get_groups_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<GroupsModel>> {
    session.permission("user::read").await?;

    let groups = state
        .group_service
        .find_by_user(&login)
        .await?;

    Ok(ApiResponse::Data(groups))
}

pub async fn user_set_groups_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<GroupsModel>,
) -> Result<ApiResponse<GroupsModel>> {
    session.permission("user::write").await?;

    let user_model = state.user_service.find_by_login(&login).await?;

    state.user_service.groups_drop(&user_model.id).await?;

    for group in payload.groups {
        match state.group_service.find_by_slug(&group).await {
            Ok(value) => {
                state.user_service.group_assign(&user_model.id, &value.id).await?
            }
            _ => warn!("can't find group -> {group}")
        }
    }

    let groups = state
        .group_service
        .find_by_user(&login)
        .await?;

    Ok(ApiResponse::Data(groups))
}