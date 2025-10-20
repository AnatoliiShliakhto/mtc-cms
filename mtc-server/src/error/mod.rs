use validator::ValidationErrors;
use crate::prelude::*;

mod generic;
mod database;
mod session;
mod smtp;

pub(crate) mod prelude {
    pub(crate) use super::{
        database::*,
        generic::*,
        session::*,
        smtp::*,
        Error,
    };
}

#[derive(thiserror::Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error(transparent)]
    GenericError(#[from] GenericError),
    #[error(transparent)]
    SessionError(#[from] SessionError),
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
    #[error[transparent]]
    SurrealDbError(#[from] surrealdb::Error),
    #[error[transparent]]
    SmtpError(#[from] SmtpError),
    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),
    #[error[transparent]]
    FormatError(#[from] core::fmt::Error),
    #[error[transparent]]
    IoError(#[from] std::io::Error),
    #[error[transparent]]
    SerdeJsonError(#[from] serde_json::Error),
    #[error[transparent]]
    JsonRejection(#[from] JsonRejection),
    #[error[transparent]]
    FormRejection(#[from] FormRejection),
    #[error[transparent]]
    MultipartError(#[from] axum::extract::multipart::MultipartError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::SessionError(error) => error.into_response(),
            Error::DatabaseError(error) => error.into_response(),
            Error::GenericError(error) => error.into_response(),
            Error::ValidationError(error) => {
                error!("{error}");
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "message": "error-bad-request", "errors": error.to_string() })),
                ).into_response()
            }
            Error::MultipartError(..)
            | Error::FormRejection(..)
            | Error::JsonRejection(..) => {
                error!("{self}");
                (StatusCode::BAD_REQUEST, Json(json!({ "message": "error-bad-request" }))).into_response()
            }
            _ => {
                error!("{self}");
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "error-internal" }))).into_response()
            }
        }
    }
}