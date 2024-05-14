use std::sync::Arc;

use axum::async_trait;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use tower_sessions::Session;

use crate::error::api_error::ApiError;
use crate::error::session_error::SessionError;
use crate::model::auth_model::{AuthModel, AuthModelTrait};
use crate::provider::config_provider::SESSION_USER_KEY;
use crate::service::user_service::UserServiceTrait;
use crate::state::AppState;

pub async fn middleware_auth_handler(
    state: State<Arc<AppState>>,
    session: Session,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, ApiError> {
    if session.is_empty().await {
        let anon_user = AuthModel {
            id: "anonymous".to_string(),
            roles: vec!["anonymous".to_string()],
            groups: vec![],
            permissions: state.user_service.permissions("anonymous").await.unwrap_or(vec![]),
        };

        match session.insert(SESSION_USER_KEY, anon_user).await {
            Ok(_) => (),
            _ => Err(ApiError::from(SessionError::InvalidSessionToken))?
        };
    }

    Ok(next.run(req).await)
}

#[async_trait]
pub trait UserSession {
    async fn sign_in(&self, auth: AuthModel) -> Result<(), ApiError>;
    async fn sign_out(&self) -> Result<(), ApiError>;
    async fn role(&self, name: &str) -> Result<(), ApiError>;
    async fn group(&self, name: &str) -> Result<(), ApiError>;
    async fn permission(&self, name: &str) -> Result<(), ApiError>;
}

#[async_trait]
impl UserSession for Session {
    async fn sign_in(&self, auth: AuthModel) -> Result<(), ApiError> {
        match self.insert(SESSION_USER_KEY, auth).await {
            Ok(_) => Ok(()),
            _ => Err(ApiError::from(SessionError::InvalidSessionToken))
        }
    }

    async fn sign_out(&self) -> Result<(), ApiError> {
        match self.flush().await {
            Ok(_) => Ok(()),
            _ => Err(ApiError::from(SessionError::InvalidSessionToken))
        }
    }

    async fn role(&self, name: &str) -> Result<(), ApiError> {
        let user: AuthModel = match self.get(SESSION_USER_KEY).await.unwrap() {
            Some(value) => value,
            _ => Err(ApiError::from(SessionError::AccessForbidden))?
        };
        match user.is_role(name) {
            true => Ok(()),
            _ => Err(ApiError::from(SessionError::AccessForbidden))
        }
    }

    async fn group(&self, name: &str) -> Result<(), ApiError> {
        let user: AuthModel = match self.get(SESSION_USER_KEY).await.unwrap() {
            Some(value) => value,
            _ => Err(ApiError::from(SessionError::AccessForbidden))?
        };
        match user.is_group(name) {
            true => Ok(()),
            _ => Err(ApiError::from(SessionError::AccessForbidden))
        }
    }

    async fn permission(&self, name: &str) -> Result<(), ApiError> {
        let user: AuthModel = match self.get(SESSION_USER_KEY).await.unwrap() {
            Some(value) => value,
            _ => Err(ApiError::from(SessionError::AccessForbidden))?
        };
        match user.is_permission(name) {
            true => Ok(()),
            _ => Err(ApiError::from(SessionError::AccessForbidden))
        }
    }
}