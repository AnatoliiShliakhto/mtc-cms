use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;

use crate::error::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::request_model::{PageRequest, ValidatedPayload};
use crate::model::response_model::ApiResponse;
use crate::model::user_model::{UserAssignRolesModel, UserModel, UserUpdateModel};
use crate::paginator::ModelPagination;
use crate::paginator::ServicePaginate;
use crate::service::role_service::RoleServiceTrait;
use crate::service::user_service::UserServiceTrait;
use crate::state::AppState;

pub async fn user_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<PageRequest>,
) -> Result<ApiResponse<ModelPagination<Vec<UserModel>>>> {
    session.permission("users::read").await?;

    let user_pagination = state.user_service
        .paginate(payload.page.unwrap_or(1)).await?;

    Ok(ApiResponse::Json(user_pagination))
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


pub async fn user_assign_roles_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<UserAssignRolesModel>,
) -> Result<ApiResponse<()>> {
    session.permission("users::write").await?;

    let user_model = state.user_service.find(&id).await?;

    for role in payload.roles.iter() {
        let role_model = state.role_service.find_by_name(role).await?;
        state.user_service.assign_role(&user_model.id, &role_model.id).await?;
    }

    Ok(ApiResponse::Ok)
}

pub async fn user_assign_role_handler(
    Path((id, role)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<()>> {
    session.permission("users::write").await?;

    let role_model = state.role_service.find_by_name(&role).await?;
    let user_model = state.user_service.find(&id).await?;
    state.user_service.assign_role(&user_model.id, &role_model.id).await?;

    Ok(ApiResponse::Ok)
}