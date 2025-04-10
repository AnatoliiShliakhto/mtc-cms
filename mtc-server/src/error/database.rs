use super::*;

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("error-db-something-went-wrong")]
    SomethingWentWrong,
    #[error("error-db-already-exists")]
    EntryAlreadyExists,
    #[error("error-db-not-found")]
    EntryNotFound,
}

impl IntoResponse for DatabaseError {
    fn into_response(self) -> Response {
        let status_code = match self {
            DatabaseError::SomethingWentWrong => StatusCode::INTERNAL_SERVER_ERROR,
            DatabaseError::EntryAlreadyExists
            | DatabaseError::EntryNotFound => StatusCode::CONFLICT,
        };

        (status_code, Json(json!({ "message": self.to_string() }))).into_response()
    }
}
