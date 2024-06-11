use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::model::response_model::ApiErrorResponse;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("errors.invalid_session")]
    InvalidSession,
    #[error("errors.access_forbidden")]
    AccessForbidden,
    #[error("errors.invalid_credentials")]
    InvalidCredentials,
    #[error("errors.user_blocked")]
    UserBlocked,
    #[error("errors.password_hash")]
    PasswordHash,
}

impl IntoResponse for SessionError {
    fn into_response(self) -> Response {
        let status_code = match self {
            SessionError::InvalidSession
            | SessionError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            SessionError::PasswordHash => StatusCode::INTERNAL_SERVER_ERROR,
            SessionError::UserBlocked
            | SessionError::AccessForbidden => StatusCode::FORBIDDEN,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
