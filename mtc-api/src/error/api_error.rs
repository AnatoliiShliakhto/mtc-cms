use axum::extract::rejection::JsonRejection;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::error::db_error::DbError;
use crate::error::session_error::SessionError;
use crate::model::response_model::ApiErrorResponse;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    SessionError(#[from] SessionError),
    #[error(transparent)]
    DbError(#[from] DbError),
    #[error("Something went wrong: {0}")]
    Generic(String),
    #[error(transparent)]
    JsonRejection(JsonRejection),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::SessionError(error) => error.into_response(),
            ApiError::DbError(error) => error.into_response(),
            ApiError::Generic(error) => error.into_response(),
            ApiError::JsonRejection(rejection) =>
                ApiErrorResponse::send(rejection.status().as_u16(),
                                       Some(rejection.body_text().to_string())),
        }
    }
}

impl From<surrealdb::Error> for ApiError {
    fn from(_val: surrealdb::Error) -> Self {
        ApiError::from(DbError::SomethingWentWrong(_val.to_string()))
    }
}

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}