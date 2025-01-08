use super::*;

#[derive(thiserror::Error, Debug)]
pub enum GenericError {
    #[error("error-internal")]
    InternalError,
    #[error("error-conflict")]
    ConflictError,
    #[error("error-bad-request")]
    BadRequest,
    #[error("error-validation")]
    ValidationError,
    #[error("error-unsupported-media-type")]
    UnsupportedMediaType,
}

impl IntoResponse for GenericError {
    /// Converts the error into a response with a JSON body containing
    /// a "message" string.
    ///
    /// The HTTP status code is determined by the error variant:
    ///
    /// - `GenericError::InternalError`: 500 Internal Server Error
    /// - `GenericError::ConflictError`: 409 Conflict
    /// - `GenericError::BadRequest`
    /// - `GenericError::ValidationError`: 400 Bad Request
    /// - `GenericError::UnsupportedMediaType`: 415 Unsupported Media Type
    fn into_response(self) -> Response {
        let status_code = match self {
            GenericError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            GenericError::ConflictError => StatusCode::CONFLICT,
            GenericError::BadRequest
            | GenericError::ValidationError => StatusCode::BAD_REQUEST,
            GenericError::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
        };

        (status_code, Json(json!({ "message": self.to_string() }))).into_response()
    }
}