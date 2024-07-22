use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;
use tracing::error;

use mtc_model::group_model::{GroupCreateModel, GroupModel, GroupUpdateModel, GroupsModel, GroupsWithTitleModel};
use mtc_model::pagination_model::{PaginationBuilder, PaginationModel};

use crate::handler::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::request_model::ValidatedPayload;
use crate::model::response_model::{ApiResponse, HandlerResult};
use crate::repository::group_repository::GroupRepositoryTrait;
use crate::repository::RepositoryPaginate;
use crate::state::AppState;

pub async fn group_all_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<GroupsModel> {
    session.permission("group::read").await?;

    state.group_service.all().await?.ok_model()
}

pub async fn group_all_title_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<GroupsWithTitleModel> {
    session.permission("group::read").await?;

    state.group_service.all_title().await?.ok_model()
}

pub async fn group_list_handler(
    page: Option<Path<usize>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<Vec<GroupModel>> {
    session.permission("group::read").await?;

    let page: usize = match page {
        Some(Path(value)) => value,
        _ => 1,
    };

    let pagination = PaginationModel::new(
        state.group_service.get_total().await?,
        state.cfg.rows_per_page,
    )
    .page(page);

    state
        .group_service
        .get_page(pagination.from, pagination.per_page)
        .await?
        .ok_page(pagination)
}

pub async fn group_get_handler(
    Path(slug): Path<String>,
    session: Session,
    state: State<Arc<AppState>>,
) -> Result<GroupModel> {
    session.permission("group::read").await?;

    state.group_service.find_by_slug(&slug).await?.ok_model()
}

pub async fn group_create_handler(
    Path(slug): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<GroupCreateModel>,
) -> Result<GroupModel> {
    session.permission("group::write").await?;

    state
        .group_service
        .create(&session.auth_id().await?, &slug, payload)
        .await?
        .ok_model()
}

pub async fn group_update_handler(
    Path(slug): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<GroupUpdateModel>,
) -> Result<GroupModel> {
    session.permission("group::write").await?;

    state
        .group_service
        .update(&session.auth_id().await?, &slug, payload)
        .await?
        .ok_model()
}

pub async fn group_delete_handler(
    Path(slug): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<()> {
    session.permission("group::delete").await?;

    state.group_service.delete(&slug).await?.ok_ok()
}

pub async fn group_list_delete_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<GroupsModel>,
) -> Result<()> {
    session.permission("group::delete").await?;

    for item in payload.groups {
        match state.group_service.delete(&item).await {
            Ok(_) => (),
            Err(e) => error!("Group delete: {}", e.to_string()),
        }
    }
    Ok(ApiResponse::Ok)
}
