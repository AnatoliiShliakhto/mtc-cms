use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;

use crate::error::api_error::ApiError;
use crate::middleware::auth_middleware::UserSession;
use crate::model::permission_model::PermissionModel;
use crate::model::response_model::ApiResponse;
use crate::service::permissions_service::PermissionsServiceTrait;
use crate::state::AppState;

pub async fn permissions_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<Vec<PermissionModel>>, ApiError> {
    session.permission("permissions::read").await?;

    let permissions = state.permissions_service.all().await?;

    Ok(ApiResponse::Data(permissions))
}

pub async fn permissions_role_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<Vec<String>>, ApiError> {
    session.permission("permissions::read").await?;

    let permissions =
        state
            .permissions_service
            .get(&id).await?;

    Ok(ApiResponse::Data(permissions))
}