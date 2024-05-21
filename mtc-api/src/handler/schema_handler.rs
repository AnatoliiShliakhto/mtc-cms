use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;

use crate::error::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::pagination_model::{PaginationBuilder, PaginationModel};
use crate::model::request_model::{PageRequest, ValidatedPayload};
use crate::model::response_model::ApiResponse;
use crate::model::schema_model::{SchemaCreateModel, SchemaModel};
use crate::repository::RepositoryPaginate;
use crate::repository::schema_repository::SchemaRepositoryTrait;
use crate::state::AppState;

pub async fn schema_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<PageRequest>,
) -> Result<ApiResponse<Vec<SchemaModel>>> {
    session.permission("schemas::read").await?;

    let pagination = PaginationModel::new(
        state.schema_service.get_total().await?,
        state.cfg.rows_per_page,
    )
        .page(payload.page.unwrap_or(1));

    let data = state.schema_service.get_page(pagination.from, pagination.per_page).await?;

    Ok(ApiResponse::DataPage(data, pagination))
}

pub async fn schema_get_handler(
    Path(id): Path<String>,
    session: Session,
    state: State<Arc<AppState>>,
) -> Result<ApiResponse<SchemaModel>> {
    session.permission("schemas::read").await?;

    let schema_model = state
        .schema_service
        .find_by_name(&id)
        .await?;

    Ok(ApiResponse::Data(schema_model))
}

pub async fn schema_create_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<SchemaCreateModel>,
) -> Result<ApiResponse<SchemaModel>> {
    session.permission("schemas::write").await?;

    let schema_model = state
        .schema_service
        .create(payload)
        .await?;

    Ok(ApiResponse::Data(schema_model))
}

pub async fn schema_delete_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<()>> {
    session.permission("schemas::delete").await?;

    state
        .schema_service
        .delete(&id)
        .await?;

    Ok(ApiResponse::Ok)
}