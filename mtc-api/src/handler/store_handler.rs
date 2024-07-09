use std::sync::Arc;

use axum::extract::{Multipart, Path, State};
use tower_sessions::Session;

use mtc_model::store_model::StoresModel;

use crate::handler::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::response_model::{ApiResponse, HandlerResult};
use crate::service::store_service::StoreTrait;
use crate::state::AppState;

pub async fn store_get_dir_handler(
    Path(path): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<StoresModel> {
    session.permission("store::read").await?;

    state.store_service.get_dir(&path).await?.ok_model()
}

pub async fn store_upload_handler(
    Path(path): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    mut multipart: Multipart,
) -> Result<()> {
    session.permission("store::write").await?;

    state.store_service.is_dir_exists_or_create(&path).await?;

    while let Some(field) = multipart.next_field().await? {
        if Some("file") == field.name() && field.file_name().is_some() {
            state.store_service.save_file(&path, field).await?
        }
    }

    Ok(ApiResponse::Ok)
}

pub async fn store_delete_handler(
    Path((path, file)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<()> {
    session.permission("store::delete").await?;

    state.store_service.delete_file(&path, &file).await?;

    Ok(ApiResponse::Ok)
}