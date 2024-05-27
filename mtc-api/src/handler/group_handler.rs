use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;

use crate::error::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::group_model::{GroupCreateModel, GroupModel, GroupUpdateModel};
use crate::model::pagination_model::{PaginationBuilder, PaginationModel};
use crate::model::request_model::ValidatedPayload;
use crate::model::response_model::ApiResponse;
use crate::repository::group_repository::GroupRepositoryTrait;
use crate::repository::RepositoryPaginate;
use crate::state::AppState;

pub async fn group_list_handler(
    page: Option<Path<usize>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<Vec<GroupModel>>> {
    session.permission("group::read").await?;

    let page: usize = match page {
        Some(Path(value)) => value,
        _ => 1
    };

    let pagination = PaginationModel::new(
        state.group_service.get_total().await?,
        state.cfg.rows_per_page,
    )
        .page(page);

    let data = state.group_service.get_page(pagination.from, pagination.per_page).await?;

    Ok(ApiResponse::DataPage(data, pagination))
}

pub async fn group_get_handler(
    Path(slug): Path<String>,
    session: Session,
    state: State<Arc<AppState>>,
) -> Result<ApiResponse<GroupModel>> {
    session.permission("group::read").await?;

    let group_model = state
        .group_service
        .find_by_slug(&slug)
        .await?;

    Ok(ApiResponse::Data(group_model))
}

pub async fn group_create_handler(
    Path(slug): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<GroupCreateModel>,
) -> Result<ApiResponse<GroupModel>> {
    session.permission("group::write").await?;

    let group_model = state
        .group_service
        .create(&slug, payload)
        .await?;

    Ok(ApiResponse::Data(group_model))
}

pub async fn group_update_handler(
    Path(slug): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<GroupUpdateModel>,
) -> Result<ApiResponse<GroupModel>> {
    session.permission("group::write").await?;

    let group_model = state
        .group_service
        .update(&slug, payload)
        .await?;

    Ok(ApiResponse::Data(group_model))
}

pub async fn group_delete_handler(
    Path(slug): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<ApiResponse<()>> {
    session.permission("group::delete").await?;

    state
        .group_service
        .delete(&slug)
        .await?;

    Ok(ApiResponse::Ok)
}