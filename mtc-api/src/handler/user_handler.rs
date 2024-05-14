use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;

use crate::error::api_error::ApiError;
use crate::middleware::auth_middleware::UserSession;
use crate::model::request_model::{ApiJson, PageRequest};
use crate::model::response_model::ApiResponse;
use crate::model::user_model::{UserModel, UserUpdateModel};
use crate::paginator::ModelPagination;
use crate::paginator::ServicePaginate;
use crate::service::user_service::UserServiceTrait;
use crate::state::AppState;

pub async fn user_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ApiJson(payload): ApiJson<PageRequest>,
) -> Result<ApiResponse<ModelPagination<Vec<UserModel>>>, ApiError> {
    session.permission("users::read").await?;

    let user_pagination = state.user_service
        .paginate(payload.page.unwrap_or(1)).await?;

    Ok(ApiResponse::Json(user_pagination))
}

pub async fn user_get_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<UserModel>, ApiError> {
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
    ApiJson(payload): ApiJson<UserUpdateModel>,
) -> Result<ApiResponse<UserModel>, ApiError> {
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
) -> Result<ApiResponse<()>, ApiError> {
    session.permission("users::delete").await?;

    state
        .user_service
        .delete(&id)
        .await?;

    Ok(ApiResponse::Ok)
}