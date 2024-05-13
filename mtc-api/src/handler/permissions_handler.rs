use std::sync::Arc;

use axum::extract::{Path, State};

use crate::error::api_error::ApiError;
use crate::model::permission_model::PermissionModel;
use crate::model::response_model::ApiResponse;
use crate::service::permissions_service::PermissionsServiceTrait;
use crate::state::AppState;

pub async fn permissions_list_handler(
    state: State<Arc<AppState>>
) -> Result<ApiResponse<Vec<PermissionModel>>, ApiError> {
    let permissions = state.permissions_service.all().await?;

    Ok(ApiResponse::Data(permissions))
}

pub async fn permissions_role_handler(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
) -> Result<ApiResponse<Vec<String>>, ApiError> {
    let permissions =
        state
            .permissions_service
            .get(&id).await?;

    Ok(ApiResponse::Data(permissions))
}