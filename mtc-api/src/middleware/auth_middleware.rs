use std::sync::Arc;

use axum::async_trait;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use tower_sessions::Session;

use mtc_model::auth_model::{AuthModel, AuthModelTrait};
use mtc_model::list_model::StringListModel;

use crate::error::api_error::ApiError;
use crate::error::session_error::SessionError;
use crate::error::Result;
use crate::model::access_model::AccessModel;
use crate::provider::config_provider::{SESSION_ACCESS_KEY, SESSION_USER_KEY};
use crate::repository::permissions_repository::PermissionsRepositoryTrait;
use crate::state::AppState;

pub async fn middleware_auth_handler(
    state: State<Arc<AppState>>,
    session: Session,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse> {
    if session.is_empty().await {
        let anon_user = session.anonymous(&state).await?;

        match session.insert(SESSION_USER_KEY, anon_user).await {
            Ok(_) => (),
            _ => Err(ApiError::from(SessionError::InvalidSession))?,
        };
    }

    Ok(next.run(req).await)
}

#[async_trait]
pub trait UserSession {
    async fn sign_in(&self, auth: AuthModel) -> Result<()>;
    async fn credentials(&self) -> Result<AuthModel>;
    async fn anonymous(&self, state: &State<Arc<AppState>>) -> Result<AuthModel>;
    async fn role(&self, slug: &str) -> Result<()>;
    async fn group(&self, slug: &str) -> Result<()>;
    async fn permission(&self, slug: &str) -> Result<()>;
    async fn auth_id(&self) -> Result<String>;
    async fn is_admin(&self) -> Result<bool>;
    async fn set_access(&self, access: AccessModel) -> Result<()>;
    async fn get_access(&self) -> Result<AccessModel>;
}

#[async_trait]
impl UserSession for Session {
    async fn sign_in(&self, auth: AuthModel) -> Result<()> {
        Ok(self.insert(SESSION_USER_KEY, auth).await?)
    }

    async fn credentials(&self) -> Result<AuthModel> {
        self.get::<AuthModel>(SESSION_USER_KEY)
            .await?
            .ok_or(SessionError::InvalidSession.into())
    }

    async fn anonymous(&self, state: &State<Arc<AppState>>) -> Result<AuthModel> {
        Ok(AuthModel {
            id: "anonymous".to_string(),
            roles: vec!["anonymous".to_string()],
            groups: vec![],
            permissions: state
                .permissions_service
                .find_by_user("anonymous")
                .await
                .unwrap_or(StringListModel {
                    list: vec!["content::read".to_string()],
                })
                .list,
        })
    }

    async fn role(&self, slug: &str) -> Result<()> {
        match self
            .get::<AuthModel>(SESSION_USER_KEY)
            .await?
            .ok_or(ApiError::from(SessionError::InvalidSession))?
            .is_role(slug)
        {
            true => Ok(()),
            _ => Err(ApiError::from(SessionError::AccessForbidden)),
        }
    }

    async fn group(&self, slug: &str) -> Result<()> {
        match self
            .get::<AuthModel>(SESSION_USER_KEY)
            .await?
            .ok_or(ApiError::from(SessionError::InvalidSession))?
            .is_group(slug)
        {
            true => Ok(()),
            _ => Err(ApiError::from(SessionError::AccessForbidden)),
        }
    }

    async fn permission(&self, slug: &str) -> Result<()> {
        match self
            .get::<AuthModel>(SESSION_USER_KEY)
            .await?
            .ok_or(ApiError::from(SessionError::InvalidSession))?
            .is_permission(slug)
        {
            true => Ok(()),
            _ => Err(ApiError::from(SessionError::AccessForbidden)),
        }
    }

    async fn auth_id(&self) -> Result<String> {
        Ok(self
            .get::<AuthModel>(SESSION_USER_KEY)
            .await?
            .ok_or(ApiError::from(SessionError::InvalidSession))?
            .id)
    }

    async fn is_admin(&self) -> Result<bool> {
        Ok(self
            .get::<AuthModel>(SESSION_USER_KEY)
            .await?
            .ok_or(ApiError::from(SessionError::InvalidSession))?
            .is_admin())
    }

    async fn set_access(&self, access: AccessModel) -> Result<()> {
        Ok(self.insert(SESSION_ACCESS_KEY, access).await?)
    }

    async fn get_access(&self) -> Result<AccessModel> {
        Ok(self
            .get::<AccessModel>(SESSION_ACCESS_KEY)
            .await?
            .ok_or(ApiError::from(SessionError::InvalidSession))?)
    }
}
