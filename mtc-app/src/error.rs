use super::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Generic(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Network(#[from] reqwest::Error),
    #[error("failed to parse as string: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

#[derive(serde::Serialize)]
#[serde(tag = "kind", content = "message")]
#[serde(rename_all = "camelCase")]
pub enum ErrorKind {
    Generic(String),
    Io(String),
    Network(String),
    Utf8(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let error_message = self.to_string();
        let error_kind = match self {
            Self::Generic(_) => ErrorKind::Generic(error_message),
            Self::Io(_) => ErrorKind::Io(error_message),
            Self::Network(_) => ErrorKind::Network(error_message),
            Self::Utf8(_) => ErrorKind::Utf8(error_message),
        };
        error_kind.serialize(serializer)
    }
}