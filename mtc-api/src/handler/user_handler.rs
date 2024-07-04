use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;
use tracing::{error, warn};

use mtc_model::group_model::GroupsModel;
use mtc_model::pagination_model::{PaginationBuilder, PaginationModel};
use mtc_model::permission_model::PermissionsModel;
use mtc_model::role_model::RolesModel;
use mtc_model::user_model::{UserCreateModel, UserModel, UserUpdateModel, UsersModel};

use crate::handler::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::request_model::ValidatedPayload;
use crate::model::response_model::{ApiResponse, HandlerResult};
use crate::repository::group_repository::GroupRepositoryTrait;
use crate::repository::permissions_repository::PermissionsRepositoryTrait;
use crate::repository::role_repository::RoleRepositoryTrait;
use crate::repository::user_repository::UserRepositoryTrait;
use crate::repository::RepositoryPaginate;
use crate::state::AppState;

pub async fn user_list_handler(
    page: Option<Path<usize>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<Vec<UserModel>> {
    session.permission("user::read").await?;

    let page: usize = match page {
        Some(Path(value)) => value,
        _ => 1,
    };

    let pagination = PaginationModel::new(
        state.user_service.get_total().await?,
        state.cfg.rows_per_page,
    )
    .page(page);

    state
        .user_service
        .get_page(pagination.from, pagination.per_page)
        .await?
        .ok_page(pagination)
}

pub async fn user_get_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<UserModel> {
    session.permission("user::read").await?;

    state.user_service.find_by_login(&login).await?.ok_model()
}

pub async fn user_create_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<UserCreateModel>,
) -> Result<UserModel> {
    session.permission("user::write").await?;

    let user_model = state
        .user_service
        .create(&session.auth_id().await?, &login, &payload)
        .await?;

    if let Some(roles) = payload.roles {
        set_roles(&state, &user_model.id, roles).await?;
    }

    if let Some(groups) = payload.groups {
        set_groups(&state, &user_model.id, groups).await?;
    }

    user_model.ok_model()
}

pub async fn user_update_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<UserUpdateModel>,
) -> Result<UserModel> {
    session.permission("user::write").await?;

    let user_model = state
        .user_service
        .update(&session.auth_id().await?, &login, &payload)
        .await?;
    state.user_service.roles_drop(&user_model.id).await?;
    state.user_service.groups_drop(&user_model.id).await?;

    if let Some(roles) = payload.roles {
        set_roles(&state, &user_model.id, roles).await?;
    }

    if let Some(groups) = payload.groups {
        set_groups(&state, &user_model.id, groups).await?;
    }

    user_model.ok_model()
}

pub async fn user_delete_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<()> {
    session.permission("user::delete").await?;

    state.user_service.delete(&login).await?.ok_ok()
}

pub async fn user_list_delete_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<UsersModel>,
) -> Result<()> {
    session.permission("user::delete").await?;

    for item in payload.users {
        match state.user_service.delete(&item).await {
            Ok(_) => (),
            Err(e) => error!("User delete: {}", e.to_string()),
        }
    }
    Ok(ApiResponse::Ok)
}

pub async fn user_get_roles_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<RolesModel> {
    session.permission("user::read").await?;

    state.role_service.find_by_user(&login).await?.ok_model()
}

pub async fn user_set_roles_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<RolesModel>,
) -> Result<RolesModel> {
    session.permission("user::write").await?;

    let user_model = state.user_service.find_by_login(&login).await?;

    state.user_service.roles_drop(&user_model.id).await?;

    set_roles(&state, &user_model.id, payload.roles).await?;

    state.role_service.find_by_user(&login).await?.ok_model()
}

pub async fn user_get_permissions_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<PermissionsModel> {
    session.permission("user::read").await?;

    state
        .permissions_service
        .find_by_user(&login)
        .await?
        .ok_model()
}

pub async fn user_get_groups_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<GroupsModel> {
    session.permission("user::read").await?;

    state.group_service.find_by_user(&login).await?.ok_model()
}

pub async fn user_set_groups_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<GroupsModel>,
) -> Result<GroupsModel> {
    session.permission("user::write").await?;

    let user_model = state.user_service.find_by_login(&login).await?;

    state.user_service.groups_drop(&user_model.id).await?;

    set_groups(&state, &user_model.id, payload.groups).await?;

    state.group_service.find_by_user(&login).await?.ok_model()
}

// coroutine

async fn set_roles(state: &Arc<AppState>, user_id: &str, roles: Vec<String>) -> Result<()> {
    for role in roles {
        match state.role_service.find_by_slug(&role).await {
            Ok(value) => state.user_service.role_assign(user_id, &value.id).await?,
            _ => warn!("can't find role -> {role}"),
        }
    }

    Ok(ApiResponse::Ok)
}

async fn set_groups(state: &Arc<AppState>, user_id: &str, groups: Vec<String>) -> Result<()> {
    for group in groups {
        match state.group_service.find_by_slug(&group).await {
            Ok(value) => state.user_service.group_assign(user_id, &value.id).await?,
            _ => warn!("can't find group -> {group}"),
        }
    }

    Ok(ApiResponse::Ok)
}

pub async fn user_block_toggle_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<()> {
    session.permission("user::write").await?;

    state
        .user_service
        .block_toggle(&session.auth_id().await?, &login)
        .await?
        .ok_ok()
}
