use dioxus::prelude::*;
use tracing::error;

use crate::error::api_error::ApiError;
use crate::global_signal::APP_ERROR;

pub mod health_service;
pub mod auth_service;

pub fn assign_error(error: ApiError) {
    match &error {
        ApiError::NetworkError(message) => error!("NetworkError: {}", message),
        ApiError::ResponseError(message) => error!("ApiResponseError: {}", message),
    }

    let message = error.message().clone();

    if *APP_ERROR.read().to_string() != message {
        *APP_ERROR.write() = message
    }
}