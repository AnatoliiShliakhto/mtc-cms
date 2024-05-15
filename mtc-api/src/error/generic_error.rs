use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::model::response_model::ApiErrorResponse;

#[derive(Error, Debug)]
pub enum GenericError {
    #[error("Internal server error")]
    SomethingWentWrong,
    #[error("{0}")]
    ConflictError(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("Unsupported media type")]
    UnsupportedMediaType,
}

impl IntoResponse for GenericError {
    fn into_response(self) -> Response {
        let status_code = match self {
            GenericError::SomethingWentWrong => StatusCode::INTERNAL_SERVER_ERROR,
            GenericError::ConflictError(..) => StatusCode::CONFLICT,
            GenericError::BadRequest(..) => StatusCode::BAD_REQUEST,
            GenericError::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}