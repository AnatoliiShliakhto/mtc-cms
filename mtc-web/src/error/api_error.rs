use dioxus_std::i18n::use_i18;
use dioxus_std::translate;
use tracing::error;

#[derive(Debug)]
pub enum ApiError {
    NetworkError(String),
    ResponseError(String),
}

impl ApiError {
    pub fn message(&self) -> String {
        let i18 = use_i18();

        match self {
            ApiError::NetworkError(message) => translate!(i18, message),
            ApiError::ResponseError(message) => translate!(i18, message),
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        error!("Network error: {}", value.to_string());
        ApiError::NetworkError("errors.connection".to_string())
    }
}

impl Into<String> for ApiError {
    fn into(self) -> String {
        match self {
            ApiError::NetworkError(message) => message,
            ApiError::ResponseError(message) => message,
        }
    }
}