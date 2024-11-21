use crate::prelude::*;

mod generic;
mod database;
mod session;

pub mod prelude {
    pub use {
        super::{
            Error,
            database::*,
            generic::*,
            session::*,
        }
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
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::SessionError(error) => error.into_response(),
            Error::DatabaseError(error) => error.into_response(),
            Error::GenericError(error) => error.into_response(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        error!(target: "I/O", "{error}");
        Self::from(GenericError::InternalError)
    }
}

impl From<surrealdb::Error> for Error {
    fn from(e: surrealdb::Error) -> Self {
        error!(target: "SurrealDB", "{e}");

        match e {
            surrealdb::Error::Db(e) => {
                match e {
                    surrealdb::error::Db::RecordExists { .. }
                    | surrealdb::error::Db::IndexExists { .. }
                    => Self::from(DatabaseError::EntryAlreadyExists),
                    _ => Self::from(DatabaseError::SomethingWentWrong),
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
            surrealdb::Error::Api(_) => {
                Self::from(DatabaseError::SomethingWentWrong)
            }
        }
    }
}

/*
impl From<tower_sessions::session::Error> for Error {
    fn from(err: tower_sessions::session::Error) -> Self {
        error!(target: "session", "{err}");
        Self::from(SessionError::InvalidSession)
    }
}

 */

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        error!(target: "serde_json", "{err}");
        Self::from(GenericError::ConflictError)
    }
}

impl From<JsonRejection> for Error {
    fn from(rejection: JsonRejection) -> Self {
        error!("Rejection: {rejection:#?}");
        Self::from(GenericError::BadRequest)
    }
}

impl From<FormRejection> for Error {
    fn from(rejection: FormRejection) -> Self {
        error!("Rejection: {rejection:#?}");
        Self::from(GenericError::BadRequest)
    }
}

impl From<axum::extract::multipart::MultipartError> for Error {
    fn from(err: axum::extract::multipart::MultipartError) -> Self {
        error!("Upload: {err:#?}");
        Self::from(GenericError::BadRequest)
    }
}