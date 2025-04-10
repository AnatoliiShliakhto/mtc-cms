use super::*;

pub trait SessionTrait {
    async fn set_state(&self, auth: &AuthState, access: &Access) -> Result<()>;
    async fn get_auth_id(&self) -> Result<Cow<'static, str>>;
    async fn get_auth_login(&self) -> Result<Cow<'static, str>>;
    async fn get_auth_state(&self) -> Result<AuthState>;
    async fn get_access_state(&self) -> Result<Access>;
    async fn has_permission(&self, permission: &str) -> Result<()>;
}

impl SessionTrait for Session {

    async fn set_state(&self, auth: &AuthState, access: &Access) -> Result<()> {
        if self.get_session_id().to_string().ne(SESSION_NULL_ID) {
            self.set(SESSION_AUTH_KEY, auth);
            self.set(SESSION_ACCESS_KEY, access);
        }

        Ok(())
    }


    async fn get_auth_id(&self) -> Result<Cow<'static, str>> {
        Ok(self.get::<AuthState>(SESSION_AUTH_KEY).unwrap_or_default().id)
    }

    async fn get_auth_login(&self) -> Result<Cow<'static, str>> {
        Ok(self.get::<AuthState>(SESSION_AUTH_KEY).unwrap_or_default().login)
    }

    async fn get_auth_state(&self) -> Result<AuthState> {
        Ok(self.get::<AuthState>(SESSION_AUTH_KEY).unwrap_or_default())
    }

    async fn get_access_state(&self) -> Result<Access> {
        Ok(self.get::<Access>(SESSION_ACCESS_KEY).unwrap_or_default())
    }

    async fn has_permission(&self, permission: &str) -> Result<()> {
        match self.get::<AuthState>(SESSION_AUTH_KEY)
            .unwrap_or_default()
            .has_permission(permission) {
            true => Ok(()),
            _ => Err(SessionError::AccessForbidden)?
        }
    }
}
