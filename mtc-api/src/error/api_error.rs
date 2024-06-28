use axum::extract::rejection::{FormRejection, JsonRejection};
use axum::response::{IntoResponse, Response};
use surrealdb::Error;
use thiserror::Error;
use tracing::error;

use crate::error::db_error::DbError;
use crate::error::generic_error::GenericError;
use crate::error::session_error::SessionError;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
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
        
        match err {
            Error::Db(db_error) => {
                match db_error {
                    surrealdb::error::Db::RecordExists { .. }
                    | surrealdb::error::Db::IndexExists { .. } 
                    | surrealdb::error::Db::TxKeyAlreadyExistsCategory { .. } => Self::from(DbError::EntryAlreadyExists),
                    _ => Self::from(DbError::SomethingWentWrong),
/*                    
                    Error::Ignore => {}
                    Error::Break => {}
                    Error::Continue => {}
                    Error::RetryWithId(_) => {}
                    Error::Unreachable(_) => {}
                    Error::Deprecated(_) => {}
                    Error::Thrown(_) => {}
                    Error::Ds(_) => {}
                    Error::Tx(_) => {}
                    Error::TxFailure => {}
                    Error::TxFinished => {}
                    Error::TxReadonly => {}
                    Error::TxConditionNotMet => {}
                    Error::TxKeyAlreadyExists => {}
                    Error::TxKeyTooLarge => {}
                    Error::TxValueTooLarge => {}
                    Error::TxTooLarge => {}
                    Error::NsEmpty => {}
                    Error::DbEmpty => {}
                    Error::QueryEmpty => {}
                    Error::QueryRemaining => {}
                    Error::InvalidQuery(_) => {}
                    Error::InvalidContent { .. } => {}
                    Error::InvalidMerge { .. } => {}
                    Error::InvalidPatch { .. } => {}
                    Error::PatchTest { .. } => {}
                    Error::HttpDisabled => {}
                    Error::InvalidParam { .. } => {}
                    Error::InvalidField { .. } => {}
                    Error::InvalidSplit { .. } => {}
                    Error::InvalidOrder { .. } => {}
                    Error::InvalidGroup { .. } => {}
                    Error::InvalidLimit { .. } => {}
                    Error::InvalidStart { .. } => {}
                    Error::InvalidScript { .. } => {}
                    Error::InvalidModel { .. } => {}
                    Error::InvalidFunction { .. } => {}
                    Error::InvalidArguments { .. } => {}
                    Error::InvalidUrl(_) => {}
                    Error::InvalidVectorDimension { .. } => {}
                    Error::InvalidVectorDistance { .. } => {}
                    Error::InvalidVectorType { .. } => {}
                    Error::InvalidVectorValue(_) => {}
                    Error::InvalidRegex(_) => {}
                    Error::InvalidTimeout(_) => {}
                    Error::QueryTimedout => {}
                    Error::QueryCancelled => {}
                    Error::QueryNotExecuted => {}
                    Error::QueryNotExecutedDetail { .. } => {}
                    Error::NsNotAllowed { .. } => {}
                    Error::DbNotAllowed { .. } => {}
                    Error::NsNotFound { .. } => {}
                    Error::NtNotFound { .. } => {}
                    Error::NlNotFound { .. } => {}
                    Error::DbNotFound { .. } => {}
                    Error::DtNotFound { .. } => {}
                    Error::DlNotFound { .. } => {}
                    Error::EvNotFound { .. } => {}
                    Error::FcNotFound { .. } => {}
                    Error::FdNotFound { .. } => {}
                    Error::MlNotFound { .. } => {}
                    Error::ScNotFound { .. } => {}
                    Error::ClAlreadyExists { .. } => {}
                    Error::NdNotFound { .. } => {}
                    Error::StNotFound { .. } => {}
                    Error::PaNotFound { .. } => {}
                    Error::TbNotFound { .. } => {}
                    Error::LvNotFound { .. } => {}
                    Error::LqNotFound { .. } => {}
                    Error::AzNotFound { .. } => {}
                    Error::IxNotFound { .. } => {}
                    Error::UnsupportedDistance(_) => {}
                    Error::UserRootNotFound { .. } => {}
                    Error::UserNsNotFound { .. } => {}
                    Error::UserDbNotFound { .. } => {}
                    Error::RealtimeDisabled => {}
                    Error::ComputationDepthExceeded => {}
                    Error::InvalidStatementTarget { .. } => {}
                    Error::CreateStatement { .. } => {}
                    Error::UpdateStatement { .. } => {}
                    Error::RelateStatement { .. } => {}
                    Error::DeleteStatement { .. } => {}
                    Error::InsertStatement { .. } => {}
                    Error::LiveStatement { .. } => {}
                    Error::KillStatement { .. } => {}
                    Error::SingleOnlyOutput => {}
                    Error::TablePermissions { .. } => {}
                    Error::ParamPermissions { .. } => {}
                    Error::FunctionPermissions { .. } => {}
                    Error::TableIsView { .. } => {}
                    Error::TableCheck { .. } => {}
                    Error::FieldCheck { .. } => {}
                    Error::FieldValue { .. } => {}
                    Error::FieldReadonly { .. } => {}
                    Error::IdMismatch { .. } => {}
                    Error::IdInvalid { .. } => {}
                    Error::CoerceTo { .. } => {}
                    Error::ConvertTo { .. } => {}
                    Error::LengthInvalid { .. } => {}
                    Error::TryAdd(_, _) => {}
                    Error::TrySub(_, _) => {}
                    Error::TryMul(_, _) => {}
                    Error::TryDiv(_, _) => {}
                    Error::TryRem(_, _) => {}
                    Error::TryPow(_, _) => {}
                    Error::TryNeg(_) => {}
                    Error::TryFrom(_, _) => {}
                    Error::Http(_) => {}
                    Error::Channel(_) => {}
                    Error::Io(_) => {}
                    Error::Encode(_) => {}
                    Error::Decode(_) => {}
                    Error::Revision(_) => {}
                    Error::CorruptedIndex(_) => {}
                    Error::NoIndexFoundForMatch { .. } => {}
                    Error::AnalyzerError(_) => {}
                    Error::HighlightError(_) => {}
                    Error::Bincode(_) => {}
                    Error::FstError(_) => {}
                    Error::Utf8Error(_) => {}
                    Error::ObsError(_) => {}
                    Error::ModelComputation(_) => {}
                    Error::FeatureNotYetImplemented { .. } => {}
                    Error::DuplicatedMatchRef { .. } => {}
                    Error::TimestampOverflow(_) => {}
                    Error::Internal(_) => {}
                    Error::Unimplemented(_) => {}
                    Error::CorruptedVersionstampInKey(_) => {}
                    Error::InvalidLevel(_) => {}
                    Error::IamError(_) => {}
                    Error::ScriptingNotAllowed => {}
                    Error::FunctionNotAllowed(_) => {}
                    Error::NetTargetNotAllowed(_) => {}
                    Error::TokenMakingFailed => {}
                    Error::NoRecordFound => {}
                    Error::SignupQueryFailed => {}
                    Error::SigninQueryFailed => {}
                    Error::ScopeNoSignup => {}
                    Error::ScopeNoSignin => {}
                    Error::NoScopeFound => {}
                    Error::MissingUserOrPass => {}
                    Error::NoSigninTarget => {}
                    Error::InvalidPass => {}
                    Error::InvalidAuth => {}
                    Error::InvalidSignup => {}
                    Error::UnknownAuth => {}
                    Error::MissingTokenHeader(_) => {}
                    Error::MissingTokenClaim(_) => {}
                    Error::TxKeyAlreadyExistsCategory(_) => {}
                    Error::MissingStorageEngine => {}
                    Error::AzAlreadyExists { .. } => {}
                    Error::DbAlreadyExists { .. } => {}
                    Error::EvAlreadyExists { .. } => {}
                    Error::FdAlreadyExists { .. } => {}
                    Error::FcAlreadyExists { .. } => {}
                    Error::IxAlreadyExists { .. } => {}
                    Error::MlAlreadyExists { .. } => {}
                    Error::NsAlreadyExists { .. } => {}
                    Error::PaAlreadyExists { .. } => {}
                    Error::ScAlreadyExists { .. } => {}
                    Error::TbAlreadyExists { .. } => {}
                    Error::NtAlreadyExists { .. } => {}
                    Error::DtAlreadyExists { .. } => {}
                    Error::StAlreadyExists { .. } => {}
                    Error::UserRootAlreadyExists { .. } => {}
                    Error::UserNsAlreadyExists { .. } => {}
                    Error::UserDbAlreadyExists { .. } => {}
                    Error::ExpiredSession => {}
                    Error::InvalidSessionDuration => {}
                    Error::InvalidSessionExpiration => {}
                    Error::NodeAgent(_) => {}
                    Error::LiveQueryError(_) => {}
                    
 */
                }
            }
            Error::Api(_) => {
                Self::from(DbError::SomethingWentWrong)
            }
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        error!(target: "serde_json", "{err}");
        Self::from(GenericError::ConflictError)
    }
}

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        error!("Rejection: {}", rejection.to_string());
        Self::from(GenericError::BadRequest)
    }
}

impl From<FormRejection> for ApiError {
    fn from(rejection: FormRejection) -> Self {
        error!("Rejection: {}", rejection.to_string());
        Self::from(GenericError::BadRequest)
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(_errors: validator::ValidationErrors) -> Self {
//        error!("{}", format!("Input validation error: [{errors}]").replace('\n', ", "));
        Self::from(GenericError::ValidationError)
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
        error!("Conflict error: {}", message);
        Self::from(GenericError::ConflictError)
    }
}

pub trait ToApiError {
    fn to_internal_error(self) -> ApiError;
    fn to_bad_request_error(self) -> ApiError;
    fn to_conflict_error(self) -> ApiError;
}

impl ToApiError for &str {
    fn to_internal_error(self) -> ApiError {
        error!("Internal error: {}", self.to_string());
        ApiError::from(GenericError::InternalError)
    }

    fn to_bad_request_error(self) -> ApiError {
        error!("Bad Request error: {}", self.to_string());
        ApiError::from(GenericError::BadRequest)
    }

    fn to_conflict_error(self) -> ApiError {
        error!("Conflict error: {}", self.to_string());
        ApiError::from(GenericError::ConflictError)
    }
}