use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::extract::State;
use tower_sessions::cookie::time::Duration;
use tower_sessions::{Expiry, Session};

use mtc_model::auth_model::{AuthModel, SignInModel};
use mtc_model::group_model::GroupsModel;
use mtc_model::permission_model::PermissionsModel;
use mtc_model::role_model::RolesModel;
use mtc_model::user_model::UserChangePasswordModel;

use crate::error::api_error::ApiError;
use crate::error::session_error::SessionError;
use crate::handler::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::request_model::ValidatedPayload;
use crate::model::response_model::HandlerResult;
use crate::repository::group_repository::GroupRepositoryTrait;
use crate::repository::permissions_repository::PermissionsRepositoryTrait;
use crate::repository::role_repository::RoleRepositoryTrait;
use crate::repository::user_repository::UserRepositoryTrait;
use crate::state::AppState;

pub async fn sign_in_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<SignInModel>,
) -> Result<AuthModel> {
    let user_model = match state.user_service.find_by_login(&payload.login).await {
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
    if argon2
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        Err(ApiError::from(SessionError::InvalidCredentials))?
    }

    let auth_model = AuthModel {
        id: user_model.login.clone(),
        roles: state
            .role_service
            .find_by_user(&user_model.login)
            .await
            .unwrap_or(RolesModel {
                roles: vec!["anonymous".to_string()],
            })
            .roles,
        groups: state
            .group_service
            .find_by_user(&user_model.login)
            .await
            .unwrap_or(GroupsModel { groups: vec![] })
            .groups,
        permissions: state
            .permissions_service
            .find_by_user(&user_model.login)
            .await
            .unwrap_or(PermissionsModel {
                permissions: vec![],
            })
            .permissions,
    };

    if &auth_model.id != "anonymous" {
        session.set_expiry(Some(Expiry::OnInactivity(Duration::minutes(
            state.cfg.session_expiration,
        ))));
    }
    session.sign_in(auth_model.clone()).await?;

    auth_model.ok_model()
}

pub async fn sign_out_handler(state: State<Arc<AppState>>, session: Session) -> Result<AuthModel> {
    session.flush().await?;

    session.anonymous(&state).await?.ok_model()
}

pub async fn get_credentials_handler(session: Session) -> Result<AuthModel> {
    session.credentials().await?.ok_model()
}

pub async fn change_password_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<UserChangePasswordModel>,
) -> Result<()> {
    let user_model = match state
        .user_service
        .find_by_login(&session.auth_id().await?)
        .await
    {
        Ok(value) => value,
        _ => Err(ApiError::from(SessionError::InvalidCredentials))?,
    };

    let argon2 = Argon2::default();

    let password_hash = match PasswordHash::new(&user_model.password) {
        Ok(value) => value,
        _ => Err(ApiError::from(SessionError::PasswordHash))?,
    };
    if argon2
        .verify_password(payload.old_password.as_bytes(), &password_hash)
        .is_err()
    {
        Err(ApiError::from(SessionError::InvalidCredentials))?
    }

    state
        .user_service
        .change_password(&user_model.login, &payload.new_password)
        .await?;

    user_model.ok_ok()
}
