use super::*;

#[async_trait]
pub trait SessionTrait {
    async fn sign_in(&self, auth: &AuthState, user: &str, access: &Access) -> Result<()>;
    async fn get_auth_id(&self) -> Result<Cow<'static, str>>;
    async fn get_user(&self) -> Result<Cow<'static, str>>;
    async fn get_auth_state(&self) -> Result<AuthState>;
    async fn get_access_state(&self) -> Result<Access>;
    async fn has_permission(&self, permission: &str) -> Result<()>;
}

#[async_trait]
impl SessionTrait for Session {
    async fn sign_in(&self, auth: &AuthState, user: &str, access: &Access) -> Result<()> {
        self.insert(SESSION_AUTH_KEY, auth).await?;
        self.insert(SESSION_USER, user).await?;
        self.insert(SESSION_ACCESS_KEY, access).await?;

        Ok(())
    }

    async fn get_auth_id(&self) -> Result<Cow<'static, str>> {
        Ok(self.get::<AuthState>(SESSION_AUTH_KEY).await?.unwrap_or_default().id)
    }

    async fn get_user(&self) -> Result<Cow<'static, str>> {
        Ok(self.get::<Cow<str>>(SESSION_USER).await?
            .unwrap_or(ROLE_ANONYMOUS.to_uppercase().into())
        )
    }

    async fn get_auth_state(&self) -> Result<AuthState> {
        Ok(self.get::<AuthState>(SESSION_AUTH_KEY).await?.unwrap_or_default())
    }

    async fn get_access_state(&self) -> Result<Access> {
        Ok(self.get::<Access>(SESSION_ACCESS_KEY).await?.unwrap_or_default())
    }

    async fn has_permission(&self, permission: &str) -> Result<()> {
        match self.get::<AuthState>(SESSION_AUTH_KEY)
            .await?
            .unwrap_or_default()
            .has_permission(permission) {
            true => Ok(()),
            _ => Err(SessionError::AccessForbidden)?
        }
    }
}
