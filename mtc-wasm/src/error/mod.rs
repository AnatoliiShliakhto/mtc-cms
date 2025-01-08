use crate::prelude::*;

/// Custom MTC-WASM error type
#[derive(Debug, PartialEq)]
pub enum Error {
    Generic(Cow<'static, str>),
    Network(Cow<'static, str>),
    Response(Cow<'static, str>),
    None,
}

impl Error {
    pub fn message(&self) -> Cow<'static, str> {
        match self {
            Error::Generic(message)
            | Error::Network(message)
            | Error::Response(message) => Cow::from(t!(message)),
            Error::None => "".into(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Network("error-connection".into())
    }
}

impl From<Error> for Cow<'static, str> {
    fn from(value: Error) -> Self {
        match value {
            Error::Generic(message) 
            | Error::Network(message)
            | Error::Response(message) => message,
            Error::None => "".into(),
        }
    }
}