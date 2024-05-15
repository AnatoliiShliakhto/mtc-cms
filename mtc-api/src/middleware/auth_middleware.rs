use std::sync::Arc;

use axum::async_trait;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use tower_sessions::Session;
use crate::error::api_error::ApiError;

use crate::error::Result;
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
) -> Result<impl IntoResponse> {
    if session.is_empty().await {
        let anon_user = AuthModel {
            id: "anonymous".to_string(),
            roles: vec!["anonymous".to_string()],
            groups: vec![],
            permissions: state.user_service.permissions("anonymous").await.unwrap_or(vec![]),
        };

        match session.insert(SESSION_USER_KEY, anon_user).await {
            Ok(_) => (),
            _ => Err(ApiError::from(SessionError::InvalidSession))?
        };
    }

    Ok(next.run(req).await)
}

#[async_trait]
pub trait UserSession {
    async fn sign_in(&self, auth: AuthModel) -> Result<()>;
    async fn sign_out(&self) -> Result<()>;
    async fn role(&self, name: &str) -> Result<()>;
    async fn group(&self, name: &str) -> Result<()>;
    async fn permission(&self, name: &str) -> Result<()>;
}

#[async_trait]
impl UserSession for Session {
    async fn sign_in(
        &self,
        auth: AuthModel,
    ) -> Result<()> {
        match self.insert(SESSION_USER_KEY, auth).await {
            Ok(_) => Ok(()),
            _ => Err(ApiError::from(SessionError::InvalidSession))
        }
    }

    async fn sign_out(&self) -> Result<()> {
        match self.flush().await {
            Ok(_) => Ok(()),
            _ => Err(ApiError::from(SessionError::InvalidSession))
        }
    }

    async fn role(
        &self,
        name: &str,
    ) -> Result<()> {
        match self.get::<AuthModel>(SESSION_USER_KEY)
            .await
            .unwrap()
            .unwrap()
            .is_role(name) {
            true => Ok(()),
            _ => Err(ApiError::from(SessionError::AccessForbidden))
        }
    }

    async fn group(
        &self,
        name: &str,
    ) -> Result<()> {
        match self.get::<AuthModel>(SESSION_USER_KEY)
            .await
            .unwrap()
            .unwrap()
            .is_group(name) {
            true => Ok(()),
            _ => Err(ApiError::from(SessionError::AccessForbidden))
        }
    }

    async fn permission(
        &self,
        name: &str,
    ) -> Result<()> {
        match self.get::<AuthModel>(SESSION_USER_KEY)
            .await
            .unwrap()
            .unwrap()
            .is_permission(name) {
            true => Ok(()),
            _ => Err(ApiError::from(SessionError::AccessForbidden))
        }
    }
}