use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::model::response_model::ApiErrorResponse;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Something went wrong")]
    SomethingWentWrong,
    #[error("Entry already exists")]
    EntryAlreadyExists,
    #[error("Entry not found")]
    EntryNotFound,
    #[error("Entry not updated")]
    EntryUpdate,
    #[error("Entry not deleted")]
    EntryDelete,
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let status_code = match self {
            DbError::SomethingWentWrong => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::EntryAlreadyExists
            | DbError::EntryNotFound
            | DbError::EntryUpdate
            | DbError::EntryDelete => StatusCode::CONFLICT,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
