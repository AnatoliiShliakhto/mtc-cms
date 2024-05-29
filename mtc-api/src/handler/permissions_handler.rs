use std::sync::Arc;

use axum::extract::State;
use tower_sessions::Session;

use crate::handler::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::permission_model::PermissionsModel;
use crate::model::response_model::HandlerResult;
use crate::repository::permissions_repository::PermissionsRepositoryTrait;
use crate::state::AppState;

pub async fn permissions_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<PermissionsModel> {
    session.permission("role::read").await?;

    state
        .permissions_service
        .all()
        .await?
        .ok_model()
}