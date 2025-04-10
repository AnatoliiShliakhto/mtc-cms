use super::*;

#[derive(thiserror::Error, Debug)]
pub enum SessionError {
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
    fn into_response(self) -> Response {
        let status_code = match self {
            SessionError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            SessionError::PasswordHash => StatusCode::INTERNAL_SERVER_ERROR,
            SessionError::UserBlocked
            | SessionError::AccessForbidden => StatusCode::FORBIDDEN,
        };

        (status_code, Json(json!({ "message": self.to_string() }))).into_response()
    }
}
