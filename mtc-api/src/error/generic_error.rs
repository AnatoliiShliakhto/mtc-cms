use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::model::response_model::ApiErrorResponse;

#[derive(Error, Debug)]
pub enum GenericError {
    #[error("errors.internal")]
    InternalError,
    #[error("errors.conflict")]
    ConflictError,
    #[error("errors.bad_request")]
    BadRequest,
    #[error("errors.validation")]
    ValidationError,
    #[error("errors.unsupported_media")]
    UnsupportedMediaType,
}

impl IntoResponse for GenericError {
    fn into_response(self) -> Response {
        let status_code = match self {
            GenericError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            GenericError::ConflictError => StatusCode::CONFLICT,
            GenericError::BadRequest
            | GenericError::ValidationError => StatusCode::BAD_REQUEST,
            GenericError::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}