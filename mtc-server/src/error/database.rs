use super::*;

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("error-db-something-went-wrong")]
    SomethingWentWrong,
    #[error("error-db-already-exists")]
    EntryAlreadyExists,
    #[error("error-db-not-found")]
    EntryNotFound,
    #[error("error-db-update")]
    EntryUpdate,
    #[error("error-db-delete")]
    EntryDelete,
}

impl IntoResponse for DatabaseError {
    /// Converts the `DatabaseError` into a well-formed `Response` with an
    /// appropriate status code and JSON payload describing the error.
    ///
    /// # Status Codes
    ///
    /// - `INTERNAL_SERVER_ERROR` (500) for `SomethingWentWrong`
    /// - `CONFLICT` (409) for `EntryAlreadyExists`, `EntryNotFound`,
    ///   `EntryUpdate`, and `EntryDelete`
    ///
    /// # JSON Payload
    ///
    /// The JSON payload will contain a single key `message` with the value of
    /// the error message of `self`.
    fn into_response(self) -> Response {
        let status_code = match self {
            DatabaseError::SomethingWentWrong => StatusCode::INTERNAL_SERVER_ERROR,
            DatabaseError::EntryAlreadyExists
            | DatabaseError::EntryNotFound
            | DatabaseError::EntryUpdate
            | DatabaseError::EntryDelete => StatusCode::CONFLICT,
        };

        (status_code, Json(json!({ "message": self.to_string() }))).into_response()
    }
}
