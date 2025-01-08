use super::*;

#[derive(thiserror::Error, Debug)]
pub enum SessionError {
    #[error("error-invalid-session")]
    InvalidSession,
    #[error("error-access-forbidden")]
    AccessForbidden,
    #[error("error-invalid-credentials")]
    InvalidCredentials,
    #[error("error-user-blocked")]
    UserBlocked,
    #[error("error-password-hash")]
    PasswordHash,
}

impl IntoResponse for SessionError {
    /// Converts the `SessionError` into a `Response` that can be returned to the client.
    ///
    /// The HTTP status code and JSON response body are determined as follows:
    ///
    /// - `InvalidSession` and `InvalidCredentials`: 401 Unauthorized
    /// - `PasswordHash`: 500 Internal Server Error
    /// - `UserBlocked` and `AccessForbidden`: 403 Forbidden
    ///
    /// The JSON response body will contain a single key-value pair with the key `"message"`
    /// and the value being the string representation of the error, e.g. `"error-invalid-session"`.
    fn into_response(self) -> Response {
        let status_code = match self {
            SessionError::InvalidSession
            | SessionError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            SessionError::PasswordHash => StatusCode::INTERNAL_SERVER_ERROR,
            SessionError::UserBlocked
            | SessionError::AccessForbidden => StatusCode::FORBIDDEN,
        };

        (status_code, Json(json!({ "message": self.to_string() }))).into_response()
    }
}
