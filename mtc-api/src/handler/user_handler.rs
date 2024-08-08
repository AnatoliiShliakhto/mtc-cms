use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;
use tracing::{error, warn};

use mtc_model::list_model::StringListModel;
use mtc_model::pagination_model::{PaginationBuilder, PaginationModel};
use mtc_model::user_details_model::UserDetailsStateModel;
use mtc_model::user_model::{UserCreateModel, UserModel, UserUpdateModel};

use crate::handler::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::request_model::ValidatedPayload;
use crate::model::response_model::{ApiResponse, HandlerResult};
use crate::repository::group_repository::GroupRepositoryTrait;
use crate::repository::permissions_repository::PermissionsRepositoryTrait;
use crate::repository::role_repository::RoleRepositoryTrait;
use crate::repository::user_repository::UserRepositoryTrait;
use crate::state::AppState;

pub async fn user_list_handler(
    page: Option<Path<usize>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<Vec<UserModel>> {
    session.permission("user::read").await?;
    let access = session.get_access().await?;

    let page: usize = match page {
        Some(Path(value)) => value,
        _ => 1,
    };

    let pagination = PaginationModel::new(
        state.user_service.get_total(&access).await?,
        state.cfg.rows_per_page,
    )
    .page(page);

    state
        .user_service
        .get_page(pagination.from, pagination.per_page, &access)
        .await?
        .ok_page(pagination)
}

pub async fn user_get_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<UserModel> {
    session.permission("user::read").await?;
    let access = session.get_access().await?;

    state
        .user_service
        .find_by_login(&login, &access)
        .await?
        .ok_model()
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

    let access_level = state
        .user_service
        .get_roles_max_access_level(&user_model.login)
        .await
        .unwrap_or(999);
    state
        .user_service
        .update_access_level(&user_model.login, access_level)
        .await?;

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

    let access_level = state
        .user_service
        .get_roles_max_access_level(&user_model.login)
        .await
        .unwrap_or(999);
    state
        .user_service
        .update_access_level(&user_model.login, access_level)
        .await?;

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
    ValidatedPayload(payload): ValidatedPayload<StringListModel>,
) -> Result<()> {
    session.permission("user::delete").await?;

    for item in payload.list {
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
) -> Result<StringListModel> {
    session.permission("user::read").await?;

    state.role_service.find_by_user(&login).await?.ok_model()
}

pub async fn user_set_roles_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<StringListModel>,
) -> Result<StringListModel> {
    session.permission("user::write").await?;
    let access = session.get_access().await?;

    let user_model = state.user_service.find_by_login(&login, &access).await?;

    state.user_service.roles_drop(&user_model.id).await?;

    set_roles(&state, &user_model.id, payload.list).await?;

    let access_level = state
        .user_service
        .get_roles_max_access_level(&user_model.login)
        .await
        .unwrap_or(999);
    state
        .user_service
        .update_access_level(&user_model.login, access_level)
        .await?;

    state
        .role_service
        .find_by_user(&user_model.login)
        .await?
        .ok_model()
}

pub async fn user_get_permissions_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<StringListModel> {
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
) -> Result<StringListModel> {
    session.permission("user::read").await?;

    state.group_service.find_by_user(&login).await?.ok_model()
}

pub async fn user_set_groups_handler(
    Path(login): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<StringListModel>,
) -> Result<StringListModel> {
    session.permission("user::write").await?;
    let access = session.get_access().await?;

    let user_model = state.user_service.find_by_login(&login, &access).await?;

    state.user_service.groups_drop(&user_model.id).await?;

    set_groups(&state, &user_model.id, payload.list).await?;

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

pub async fn users_get_state(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<StringListModel>,
) -> Result<Vec<UserDetailsStateModel>> {
    session.permission("user::read").await?;
    let access = session.get_access().await?;

    state
        .user_service
        .get_users_state(payload.list, &access)
        .await?
        .ok_model()
}