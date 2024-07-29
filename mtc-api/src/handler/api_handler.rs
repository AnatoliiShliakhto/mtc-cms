use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;

use mtc_model::api_model::{ApiListItemModel, ApiModel, ApiPostModel};
use mtc_model::pagination_model::{PaginationBuilder, PaginationModel};

use crate::error::api_error::ToApiError;
use crate::handler::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::request_model::{ApiPageRequest, ValidatedPayload};
use crate::model::response_model::HandlerResult;
use crate::repository::api_repository::ApiRepositoryTrait;
use crate::repository::schema_repository::SchemaRepositoryTrait;
use crate::service::storage_service::StorageTrait;
use crate::state::AppState;

pub async fn api_collection_list_handler(
    Path(api_page_request): Path<ApiPageRequest>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<Vec<ApiModel>> {
    let api = api_page_request.api;
    let page = api_page_request.page.unwrap_or(1);
    let schema_model = state.schema_service.find_by_slug(&api).await?;

    if schema_model.is_system || !schema_model.is_collection {
        Err("Isn't a collection type api end-point".to_bad_request_error())?
    }

    if !schema_model.is_public {
        session
            .permission(&[api.as_str(), "::read"].concat())
            .await?;
    }
    let is_admin = session.is_admin().await?;

    let pagination = PaginationModel::new(
        state.api_service.get_total(&api, is_admin).await?,
        state.cfg.rows_per_page,
    )
    .page(page);

    state
        .api_service
        .get_page(&api, pagination.from, pagination.per_page, is_admin)
        .await?
        .ok_page(pagination)
}

pub async fn api_get_single_handler(
    Path(api): Path<String>,
    session: Session,
    state: State<Arc<AppState>>,
) -> Result<ApiModel> {
    let schema_model = state.schema_service.find_by_slug(&api).await?;

    if schema_model.is_system || schema_model.is_collection {
        Err("Isn't a single type api end-point".to_bad_request_error())?
    }
    if !schema_model.is_public {
        session
            .permission(&[api.as_str(), "::read"].concat())
            .await?;
    }

    state
        .api_service
        .find_by_slug("singles", &api)
        .await?
        .ok_model()
}

pub async fn api_update_single_item_handler(
    Path(api): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<ApiPostModel>,
) -> Result<ApiModel> {
    let schema_model = state.schema_service.find_by_slug(&api).await?;

    if schema_model.is_public {
        session.permission("content::write").await?;
    } else {
        session
            .permission(&[api.as_str(), "::write"].concat())
            .await?;
    }

    if schema_model.is_system || schema_model.is_collection {
        Err("Isn't a single type api end-point".to_bad_request_error())?
    }

    state
        .api_service
        .update(
            &session.auth_id().await?,
            "singles",
            &schema_model.slug,
            payload,
        )
        .await?
        .ok_model()
}

pub async fn api_get_collection_item_handler(
    Path((api, slug)): Path<(String, String)>,
    session: Session,
    state: State<Arc<AppState>>,
) -> Result<ApiModel> {
    let schema_model = state.schema_service.find_by_slug(&api).await?;

    if schema_model.is_system || !schema_model.is_collection {
        Err("Isn't a collection type api end-point".to_bad_request_error())?
    }

    if !schema_model.is_public {
        session
            .permission(&[api.as_str(), "::read"].concat())
            .await?;
    }

    state
        .api_service
        .find_by_slug(&schema_model.slug, &slug)
        .await?
        .ok_model()
}

pub async fn api_create_collection_item_handler(
    Path((api, slug)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<ApiPostModel>,
) -> Result<ApiModel> {
    let schema_model = state.schema_service.find_by_slug(&api).await?;

    if schema_model.is_system || !schema_model.is_collection {
        Err("Isn't a collection type api end-point".to_bad_request_error())?
    }

    if schema_model.is_public {
        session.permission("content::write").await?;
    } else {
        session
            .permission(&[api.as_str(), "::write"].concat())
            .await?;
    }

    let api_model = state
        .api_service
        .create(
            &session.auth_id().await?,
            &schema_model.slug,
            &slug,
            payload,
        )
        .await?;

    state.storage_service.create_assets(&api_model.id).await?;

    api_model.ok_model()
}

pub async fn api_update_collection_item_handler(
    Path((api, slug)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<ApiPostModel>,
) -> Result<ApiModel> {
    let schema_model = state.schema_service.find_by_slug(&api).await?;

    if schema_model.is_system || !schema_model.is_collection {
        Err("Isn't a collection type api end-point".to_bad_request_error())?
    }

    if schema_model.is_public {
        session.permission("content::write").await?;
    } else {
        session
            .permission(&[api.as_str(), "::write"].concat())
            .await?;
    }

    state
        .api_service
        .update(
            &session.auth_id().await?,
            &schema_model.slug,
            &slug,
            payload,
        )
        .await?
        .ok_model()
}

pub async fn api_delete_collection_item_handler(
    Path((api, slug)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<()> {
    let schema_model = state.schema_service.find_by_slug(&api).await?;

    if schema_model.is_system || !schema_model.is_collection {
        Err("Isn't a collection type api end-point".to_bad_request_error())?
    }

    if schema_model.is_public {
        session.permission("content::write").await?;
    } else {
        session
            .permission(&[api.as_str(), "::delete"].concat())
            .await?;
    }

    let api_model = state
        .api_service
        .find_by_slug(&schema_model.slug, &slug)
        .await?;

    state.storage_service.delete_assets(&api_model.id).await?;

    state
        .api_service
        .delete(&schema_model.slug, &slug)
        .await?
        .ok_ok()
}

pub async fn api_get_all_single_items_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<Vec<ApiListItemModel>> {
    session.permission("administrator").await?;

    state.api_service.get_all_items("singles").await?.ok_model()
}

pub async fn api_get_all_collection_items_handler(
    Path(api): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<Vec<ApiListItemModel>> {
    session.permission("administrator").await?;

    state.api_service.get_all_items(&api).await?.ok_model()
}
