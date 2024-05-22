use std::sync::Arc;

use axum::extract::State;
use tower_sessions::Session;

use crate::error::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::permission_model::PermissionsModel;
use crate::model::response_model::ApiResponse;
use crate::repository::permissions_repository::PermissionsRepositoryTrait;
use crate::state::AppState;

pub async fn permissions_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<PermissionsModel>> {
    session.permission("role::read").await?;

    let permissions = state.permissions_service.all().await?;

    Ok(ApiResponse::Data(permissions))
}