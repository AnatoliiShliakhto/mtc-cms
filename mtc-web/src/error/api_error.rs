#[derive(Debug)]
pub enum ApiError {
    NetworkError(String),
    ResponseError(String),
}

impl ApiError {
    pub fn message(self) -> String {
        match self {
            ApiError::NetworkError(message) => message,
            ApiError::ResponseError(message) => message,
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        ApiError::NetworkError(value.to_string())
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