use super::*;

#[derive(thiserror::Error, Debug)]
pub enum GenericError {
    #[error("error-internal")]
    InternalError,
    #[error("error-conflict")]
    ConflictError,
    #[error("error-bad-request")]
    BadRequest,
    #[error("error-unsupported-media-type")]
    UnsupportedMediaType,
}

impl IntoResponse for GenericError {
    fn into_response(self) -> Response {
        let status_code = match self {
            GenericError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            GenericError::ConflictError => StatusCode::CONFLICT,
            GenericError::BadRequest => StatusCode::BAD_REQUEST,
            GenericError::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
        };

        (status_code, Json(json!({ "message": self.to_string() }))).into_response()
    }
}