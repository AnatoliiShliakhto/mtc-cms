use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;

use crate::error::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::permission_model::PermissionModel;
use crate::model::response_model::ApiResponse;
use crate::repository::permissions_repository::PermissionsRepositoryTrait;
use crate::state::AppState;

pub async fn permissions_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<Vec<PermissionModel>>> {
    session.permission("permissions::read").await?;

    let permissions = state.permissions_service.all().await?;

    Ok(ApiResponse::Data(permissions))
}

pub async fn permissions_role_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<Vec<String>>> {
    session.permission("permissions::read").await?;

    let permissions =
        state
            .permissions_service
            .find_by_role(&id).await?;

    Ok(ApiResponse::Data(permissions))
}