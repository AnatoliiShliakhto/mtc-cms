use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::model::response_model::ApiErrorResponse;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("Invalid session token")]
    InvalidSessionToken,
    #[error("Session has expired")]
    SessionExpired,
    #[error("Session token error: {0}")]
    SessionTokenError(String),
}

impl IntoResponse for SessionError {
    fn into_response(self) -> Response {
        let status_code = match self {
            SessionError::InvalidSessionToken
            | SessionError::SessionExpired => StatusCode::UNAUTHORIZED,
            SessionError::SessionTokenError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
