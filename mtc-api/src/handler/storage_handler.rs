use std::sync::Arc;

use axum::body::{Body, Bytes};
use axum::extract::{Multipart, Path, State};
use axum::http::header;
use axum::response::{AppendHeaders, IntoResponse, Response};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use mtc_model::storage_model::StoragesModel;

use crate::error::api_error::ApiError;
use crate::handler::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::response_model::{ApiResponse, HandlerResult};
use crate::service::storage_service::StorageTrait;
use crate::state::AppState;

#[derive(Deserialize, Serialize)]
pub struct FileResult { pub filename: String }

pub async fn storage_get_dir_handler(
    Path(path): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<StoragesModel> {
    session.permission("storage::read").await?;

    state
        .storage_service
        .get_dir(&state.storage_service.get_dir_path(&path))
        .await?
        .ok_model()
}

pub async fn private_storage_get_dir_handler(
    Path(path): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<StoragesModel> {
    session.permission("private_storage::read").await?;

    state
        .storage_service
        .get_dir(&state.storage_service.get_private_dir_path(&path))
        .await?
        .ok_model()
}

pub async fn storage_upload_handler(
    Path(path): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    mut multipart: Multipart,
) -> Result<FileResult> {
    session.permission("storage::write").await?;

    let path = state.storage_service.get_dir_path(&path);
    let mut filename:String = "".to_string();

    state.storage_service.is_dir_exists_or_create(&path).await?;

    while let Some(field) = multipart.next_field().await? {
        if Some("file") == field.name() && field.file_name().is_some() {
            let name = state.storage_service.save_file(&path, field).await?;
            filename = name.as_str().parse()?;
        }
    }

    Ok(ApiResponse::Data(FileResult { filename }))
}

pub async fn private_storage_upload_handler(
    Path(path): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    mut multipart: Multipart,
) -> Result<FileResult> {
    session.permission("private_storage::write").await?;

    let path = state.storage_service.get_private_dir_path(&path);
    let mut filename:String = "".to_string();

    state.storage_service.is_dir_exists_or_create(&path).await?;

    while let Some(field) = multipart.next_field().await? {
        if Some("file") == field.name() && field.file_name().is_some() {
            let name = state.storage_service.save_file(&path, field).await?;
            filename = name.as_str().parse()?;
        }
    }

    Ok(ApiResponse::Data(FileResult { filename }))
}

pub async fn storage_delete_handler(
    Path((path, file)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<()> {
    session.permission("storage::delete").await?;

    state
        .storage_service
        .delete_file(&state.storage_service.get_file_path(&path, &file))
        .await?;

    Ok(ApiResponse::Ok)
}

pub async fn private_storage_delete_handler(
    Path((path, file)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<()> {
    session.permission("private_storage::delete").await?;

    state
        .storage_service
        .delete_file(&state.storage_service.get_private_file_path(&path, &file))
        .await?;

    Ok(ApiResponse::Ok)
}

pub async fn private_storage_get_handler(
    Path((path, file)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> core::result::Result<Response, ApiError> {
    session.permission("private_storage::read").await?;

    let file_path = state.storage_service.get_private_file_path(&path, &file);

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
