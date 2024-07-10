use std::sync::Arc;

use axum::body::{Body, Bytes};
use axum::extract::{Multipart, Path, State};
use axum::http::header;
use axum::response::{AppendHeaders, IntoResponse, Response};
use tower_sessions::Session;

use mtc_model::store_model::StoresModel;

use crate::error::api_error::ApiError;
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

    state
        .store_service
        .get_dir(&state.store_service.get_dir_path(&path))
        .await?
        .ok_model()
}

pub async fn protected_store_get_dir_handler(
    Path(path): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<StoresModel> {
    session.permission("protected::read").await?;

    state
        .store_service
        .get_dir(&state.store_service.get_protected_dir_path(&path))
        .await?
        .ok_model()
}

pub async fn store_upload_handler(
    Path(path): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    mut multipart: Multipart,
) -> Result<()> {
    session.permission("store::write").await?;

    let path = state.store_service.get_dir_path(&path);

    state.store_service.is_dir_exists_or_create(&path).await?;

    while let Some(field) = multipart.next_field().await? {
        if Some("file") == field.name() && field.file_name().is_some() {
            state.store_service.save_file(&path, field).await?
        }
    }

    Ok(ApiResponse::Ok)
}

pub async fn protected_store_upload_handler(
    Path(path): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    mut multipart: Multipart,
) -> Result<()> {
    session.permission("protected::write").await?;

    let path = state.store_service.get_protected_dir_path(&path);

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

    state
        .store_service
        .delete_file(&state.store_service.get_file_path(&path, &file))
        .await?;

    Ok(ApiResponse::Ok)
}

pub async fn protected_store_delete_handler(
    Path((path, file)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<()> {
    session.permission("protected::delete").await?;

    state
        .store_service
        .delete_file(&state.store_service.get_protected_file_path(&path, &file))
        .await?;

    Ok(ApiResponse::Ok)
}

pub async fn protected_store_get_handler(
    Path((path, file)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> core::result::Result<Response, ApiError> {
    session.permission("protected::read").await?;

    let file_path = state.store_service.get_protected_file_path(&path, &file);

    let bytes = Bytes::from(tokio::fs::read(file_path).await?);
    let body = Body::from(bytes);

    let headers = AppendHeaders([
        (
            header::CONTENT_TYPE,
            mime_guess::from_path(&file)
                .first_or_text_plain()
                .to_string(),
        ),
        (
            header::CONTENT_DISPOSITION,
            format!("inline; filename=\"{}\"", file),
        ),
    ]);

    Ok((headers, body).into_response())
}
