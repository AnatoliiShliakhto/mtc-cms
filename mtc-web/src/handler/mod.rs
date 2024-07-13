use reqwest::{Error, Response, StatusCode};
use serde::de::DeserializeOwned;

use crate::API_URL;
use crate::error::api_error::ApiError;
use crate::model::response_model::{ApiErrorResponse, ApiResponse};

pub mod health_handler;
pub mod auth_handler;
pub mod group_handler;
pub mod role_handler;
pub mod permissions_handler;
pub mod user_handler;
pub mod schema_handler;
pub mod migration_handler;

pub struct ApiHandler {
    pub api_url: String,
    pub api_client: reqwest::Client,
}

impl Default for ApiHandler {
    fn default() -> Self {
        let api_client = reqwest::Client::builder().build().unwrap();

        Self {
            api_url: API_URL.to_string(),
            api_client,
        }
    }
}

pub trait HandlerNullResponse {
    async fn consume(self) -> Result<(), ApiError>;
}

impl HandlerNullResponse for Result<Response, Error> {
    async fn consume(self) -> Result<(), ApiError> {
        match self {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    Ok(())
                } else {
                    Err(ApiError::ResponseError(response.json::<ApiErrorResponse>().await?.message
                        .unwrap_or("errors.bad_response".to_string())))
                }
            }
            Err(e) => Err(ApiError::from(e))
        }
    }
}

pub trait HandlerResponse<T: DeserializeOwned> {
    async fn consume_data(self) -> Result<T, ApiError>;
    async fn consume_page(self) -> Result<ApiResponse<T>, ApiError>;
}

impl<T: DeserializeOwned> HandlerResponse<T> for Result<Response, Error> {
    async fn consume_data(self) -> Result<T, ApiError> {
        match self {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    Ok(response.json::<ApiResponse<T>>().await?.data)
                } else {
                    Err(ApiError::ResponseError(response.json::<ApiErrorResponse>().await?.message
                        .unwrap_or("errors.bad_response".to_string())))
                }
            }
            Err(e) => Err(ApiError::from(e))
        }
    }

    async fn consume_page(self) -> Result<ApiResponse<T>, ApiError> {
        match self {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    Ok(response.json::<ApiResponse<T>>().await?)
                } else {
                    Err(ApiError::ResponseError(response.json::<ApiErrorResponse>().await?.message
                        .unwrap_or("errors.bad_response".to_string())))
                }
            }
            Err(e) => Err(ApiError::from(e))
        }
    }
}