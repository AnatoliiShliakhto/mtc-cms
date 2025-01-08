use super::*;

pub trait SessionTrait: Send {
    fn set_state(&self, auth: &AuthState, access: &Access)
        -> impl Future<Output = Result<()>> + Send;
    fn get_auth_id(&self)
        -> impl Future<Output = Result<Cow<'static, str>>> + Send;
    fn get_auth_login(&self)
        -> impl Future<Output = Result<Cow<'static, str>>> + Send;
    fn get_auth_state(&self)
        -> impl Future<Output = Result<AuthState>> + Send;
    fn get_access_state(&self)
        -> impl Future<Output = Result<Access>> + Send;
    fn has_permission(&self, permission: &str)
        -> impl Future<Output = Result<()>> + Send;
}

impl SessionTrait for Session {

    /// Stores the user's authentication state ([`AuthState`]) and [`Access`] in the session.
    async fn set_state(&self, auth: &AuthState, access: &Access) -> Result<()> {
        if self.get_session_id().to_string().ne(SESSION_NULL_ID) {
            self.set(SESSION_AUTH_KEY, auth);
            self.set(SESSION_ACCESS_KEY, access);
        }

        Ok(())
    }


    /// Returns the user's ID from the session.
    async fn get_auth_id(&self) -> Result<Cow<'static, str>> {
        Ok(self.get::<AuthState>(SESSION_AUTH_KEY).unwrap_or_default().id)
    }

    /// Returns the user's login from the session.
    async fn get_auth_login(&self) -> Result<Cow<'static, str>> {
        Ok(self.get::<AuthState>(SESSION_AUTH_KEY).unwrap_or_default().login)
    }

    /// Returns the user's authentication state ([`AuthState`]) from the session.
    async fn get_auth_state(&self) -> Result<AuthState> {
        Ok(self.get::<AuthState>(SESSION_AUTH_KEY).unwrap_or_default())
    }

    /// Returns the user's access state ([`Access`]) from the session.
    async fn get_access_state(&self) -> Result<Access> {
        Ok(self.get::<Access>(SESSION_ACCESS_KEY).unwrap_or_default())
    }

    /// Checks if the user has the given permission.
    ///
    /// # Arguments
    ///
    /// * `permission`: The name of the permission to check.
    ///
    /// # Errors
    ///
    /// * `SessionError::AccessForbidden` if the user does not have the given permission.
    ///
    /// # Returns
    ///
    /// * `Result<()>`: Returns `Ok(())` if the user has the given permission, otherwise returns `Err(SessionError::AccessForbidden)`.
    async fn has_permission(&self, permission: &str) -> Result<()> {
        match self.get::<AuthState>(SESSION_AUTH_KEY)
            .unwrap_or_default()
            .has_permission(permission) {
            true => Ok(()),
            _ => Err(SessionError::AccessForbidden)?
        }
    }
}
