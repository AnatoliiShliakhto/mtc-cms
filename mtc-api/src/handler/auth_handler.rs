use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::extract::State;
use tower_sessions::Session;

use crate::error::api_error::ApiError;
use crate::error::Result;
use crate::error::session_error::SessionError;
use crate::middleware::auth_middleware::UserSession;
use crate::model::auth_model::{AuthModel, SignInModel};
use crate::model::group_model::GroupsModel;
use crate::model::permission_model::PermissionsModel;
use crate::model::request_model::ValidatedPayload;
use crate::model::response_model::ApiResponse;
use crate::model::role_model::RolesModel;
use crate::model::user_model::UserModel;
use crate::repository::group_repository::GroupRepositoryTrait;
use crate::repository::permissions_repository::PermissionsRepositoryTrait;
use crate::repository::role_repository::RoleRepositoryTrait;
use crate::repository::user_repository::UserRepositoryTrait;
use crate::state::AppState;

pub async fn sign_in_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<SignInModel>,
) -> Result<ApiResponse<UserModel>> {
    let user_model = match state
        .user_service
        .find_by_login(&payload.login).await {
        Ok(value) => value,
        _ => Err(ApiError::from(SessionError::InvalidCredentials))?,
    };

    if user_model.blocked {
        Err(ApiError::from(SessionError::UserBlocked))?
    }

    let argon2 = Argon2::default();

    let parsed_hash = match PasswordHash::new(&user_model.password) {
        Ok(value) => value,
        _ => Err(ApiError::from(SessionError::PasswordHash))?,
    };
    if !argon2.verify_password(payload.password.as_bytes(), &parsed_hash).is_ok() {
        return Err(ApiError::from(SessionError::InvalidCredentials));
    }

    let auth_user = AuthModel {
        id: user_model.id.clone(),
        roles: state.role_service
            .find_by_user(&user_model.login)
            .await
            .unwrap_or(RolesModel { roles: vec!["anonymous".to_string()] })
            .roles,
        groups: state.group_service
            .find_by_user(&user_model.login)
            .await
            .unwrap_or(GroupsModel { groups: vec![] })
            .groups,
        permissions: state.permissions_service
            .find_by_user(&user_model.login)
            .await
            .unwrap_or(PermissionsModel { permissions: vec![] })
            .permissions,
    };

    session.sign_in(auth_user).await?;

    Ok(ApiResponse::Data(user_model))
}

pub async fn sign_out_handler(
    session: Session,
) -> Result<ApiResponse<()>> {
    session.sign_out().await?;

    Ok(ApiResponse::Ok)
}
