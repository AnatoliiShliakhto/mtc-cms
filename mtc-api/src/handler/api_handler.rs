use std::sync::Arc;

use axum::extract::{Path, State};
use tower_sessions::Session;

use crate::error::api_error::ApiError;
use crate::error::generic_error::GenericError;
use crate::error::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::api_model::{ApiModel, ApiPostModel};
use crate::model::request_model::ValidatedPayload;
use crate::model::response_model::ApiResponse;
use crate::repository::api_repository::ApiRepositoryTrait;
use crate::repository::schema_repository::SchemaRepositoryTrait;
use crate::state::AppState;

pub async fn api_get_single_handler(
    Path(api): Path<String>,
    session: Session,
    state: State<Arc<AppState>>,
) -> Result<ApiResponse<ApiModel>> {
    session.permission(&format!("{}::read", &api)).await?;

    let schema_model = state
        .schema_service
        .find_by_slug(&api)
        .await?;

    //todo: collection response (perhaps)
    if schema_model.is_system || schema_model.is_collection {
        Err(ApiError::from(GenericError::BadRequest("Isn't a single type api end-point".to_string())))?
    }

    let api_model = state
        .api_service
        .find_by_slug("singles", &api)
        .await?;

    Ok(ApiResponse::Data(api_model))
}

pub async fn api_update_single_item_handler(
    Path(api): Path<String>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<ApiPostModel>,
) -> Result<ApiResponse<ApiModel>> {
    session.permission(&format!("{}::write", &api)).await?;

    let schema_model = state
        .schema_service
        .find_by_slug(&api)
        .await?;

    if schema_model.is_system || schema_model.is_collection {
        Err(ApiError::from(GenericError::BadRequest("Isn't a single type api end-point".to_string())))?
    }

    let api_model = state
        .api_service
        .update("singles", &schema_model.slug, payload)
        .await?;

    Ok(ApiResponse::Data(api_model))
}

pub async fn api_get_collection_item_handler(
    Path((api, slug)): Path<(String, String)>,
    session: Session,
    state: State<Arc<AppState>>,
) -> Result<ApiResponse<ApiModel>> {
    session.permission(&format!("{}::read", &api)).await?;

    let schema_model = state
        .schema_service
        .find_by_slug(&api)
        .await?;

    if schema_model.is_system || !schema_model.is_collection {
        Err(ApiError::from(GenericError::BadRequest("Isn't a collection type api end-point".to_string())))?
    }

    let api_model = state
        .api_service
        .find_by_slug(&schema_model.slug, &slug)
        .await?;

    Ok(ApiResponse::Data(api_model))
}

pub async fn api_create_collection_item_handler(
    Path((api, slug)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<ApiPostModel>,
) -> Result<ApiResponse<ApiModel>> {
    session.permission(&format!("{}::write", &api)).await?;

    let schema_model = state
        .schema_service
        .find_by_slug(&api)
        .await?;

    if schema_model.is_system || !schema_model.is_collection {
        Err(ApiError::from(GenericError::BadRequest("Isn't a collection type api end-point".to_string())))?
    }

    let api_model = state
        .api_service
        .create(&schema_model.slug, &slug, payload)
        .await?;

    Ok(ApiResponse::Data(api_model))
}

pub async fn api_update_collection_item_handler(
    Path((api, slug)): Path<(String, String)>,
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<ApiPostModel>,
) -> Result<ApiResponse<ApiModel>> {
    session.permission(&format!("{}::write", &api)).await?;

    let schema_model = state
        .schema_service
        .find_by_slug(&api)
        .await?;

    if schema_model.is_system || !schema_model.is_collection {
        Err(ApiError::from(GenericError::BadRequest("Isn't a collection type api end-point".to_string())))?
    }

    let api_model = state
        .api_service
        .update(&schema_model.slug, &slug, payload)
        .await?;

    Ok(ApiResponse::Data(api_model))
}