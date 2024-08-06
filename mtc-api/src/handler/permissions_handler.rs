use std::sync::Arc;

use axum::extract::State;
use tower_sessions::Session;

use mtc_model::list_model::RecordListModel;
use mtc_model::permission_model::{PermissionDtoModel, PermissionModel};

use crate::handler::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::request_model::ValidatedPayload;
use crate::model::response_model::HandlerResult;
use crate::repository::permissions_repository::PermissionsRepositoryTrait;
use crate::state::AppState;

pub async fn permissions_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<RecordListModel> {
    session.permission("role::read").await?;

    state.permissions_service.all().await?.ok_model()
}

pub async fn permissions_get_custom(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<Vec<PermissionModel>> {
    session.permission("role::read").await?;

    state.permissions_service.get_custom().await?.ok_model()
}

pub async fn permissions_create_custom(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<PermissionDtoModel>,
) -> Result<()> {
    session.permission("role::write").await?;

    state
        .permissions_service
        .create_custom(&session.auth_id().await?, payload)
        .await?.ok_model()
}

pub async fn permissions_delete_custom(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<PermissionDtoModel>,
) -> Result<()> {
    session.permission("role::write").await?;

    state
        .permissions_service
        .delete_custom(payload)
        .await?
        .ok_model()
}