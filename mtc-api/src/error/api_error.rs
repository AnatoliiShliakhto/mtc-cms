use axum::extract::rejection::{FormRejection, JsonRejection};
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::error;

use crate::error::db_error::DbError;
use crate::error::generic_error::GenericError;
use crate::error::session_error::SessionError;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    SessionError(#[from] SessionError),
    #[error(transparent)]
    DbError(#[from] DbError),
    #[error(transparent)]
    GenericError(#[from] GenericError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::SessionError(error) => error.into_response(),
            ApiError::DbError(error) => error.into_response(),
            ApiError::GenericError(error) => error.into_response(),
        }
    }
}

impl From<surrealdb::Error> for ApiError {
    fn from(err: surrealdb::Error) -> Self {
        error!(target: "SurrealDB", "{err}");
        Self::from(DbError::SomethingWentWrong)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        error!(target: "serde_json", "{err}");
        Self::from(GenericError::ConflictError("Json deserialization error".to_string()))
    }
}

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self::from(GenericError::BadRequest(rejection.to_string()))
    }
}

impl From<FormRejection> for ApiError {
    fn from(rejection: FormRejection) -> Self {
        Self::from(GenericError::BadRequest(rejection.to_string()))
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let message = format!("Input validation error: [{errors}]").replace('\n', ", ");
        Self::from(GenericError::BadRequest(message))
    }
}

impl From<tower_sessions::session::Error> for ApiError {
    fn from(error: tower_sessions::session::Error) -> Self {
        error!(target: "session", "{error}");
        Self::from(SessionError::InvalidSession)
    }
}

impl From<&str> for ApiError {
    fn from(message: &str) -> Self {
        Self::from(GenericError::ConflictError(message.to_string()))
    }
}

pub trait ToApiError {
    fn to_internal_error(self) -> ApiError;
    fn to_bad_request_error(self) -> ApiError;
    fn to_conflict_error(self) -> ApiError;
}

impl ToApiError for &str {
    fn to_internal_error(self) -> ApiError {
        ApiError::from(GenericError::InternalError(self.to_string()))
    }

    fn to_bad_request_error(self) -> ApiError {
        ApiError::from(GenericError::BadRequest(self.to_string()))
    }

    fn to_conflict_error(self) -> ApiError {
        ApiError::from(GenericError::ConflictError(self.to_string()))
    }
}