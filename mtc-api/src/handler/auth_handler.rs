use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::extract::State;
use tower_sessions::Session;
use crate::error::api_error::ApiError;

use crate::error::Result;
use crate::error::session_error::SessionError;
use crate::middleware::auth_middleware::UserSession;
use crate::model::auth_model::{AuthModel, SignInModel};
use crate::model::request_model::ValidatedPayload;
use crate::model::response_model::ApiResponse;
use crate::model::user_model::UserModel;
use crate::service::user_service::UserServiceTrait;
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
        roles: state.user_service.roles(&user_model.id).await.unwrap_or(vec!["anonymous".to_string()]),
        groups: state.user_service.groups(&user_model.id).await.unwrap_or(vec![]),
        permissions: state.user_service.permissions(&user_model.id).await.unwrap_or(vec![]),
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
