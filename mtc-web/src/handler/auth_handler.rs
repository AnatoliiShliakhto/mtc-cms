use reqwest::StatusCode;

use mtc_model::auth_model::{AuthModel, SignInModel};

use crate::error::api_error::ApiError;
use crate::model::response_model::{ApiErrorResponse, ApiResponse};
use crate::state::AppState;

pub trait AuthHandler {
    async fn sign_in(&self, login: String, password: String) -> Result<AuthModel, ApiError>;
    async fn sign_out(&self) -> Result<AuthModel, ApiError>;
    async fn credentials(&self) -> Result<AuthModel, ApiError>;
}

impl AuthHandler for AppState {
    async fn sign_in(
        &self,
        login: String,
        password: String,
    ) -> Result<AuthModel, ApiError> {
        match self
            .api_client
            .post([&self.api_url, "auth"].join("/"))
            .json(&SignInModel { login: login.to_string(), password: password.to_string() })
            .send()
            .await
        {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    Ok(response.json::<ApiResponse<AuthModel>>().await?.data)
                } else {
                    Err(ApiError::ResponseError(response.json::<ApiErrorResponse>().await?.message
                        .unwrap_or("errors.auth".to_string())))
                }
            }
            Err(e) => Err(ApiError::from(e))
        }
    }

    async fn sign_out(
        &self,
    ) -> Result<AuthModel, ApiError> {
        match self
            .api_client
            .delete([&self.api_url, "auth"].join("/"))
            .send()
            .await
        {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    Ok(response.json::<ApiResponse<AuthModel>>().await?.data)
                } else {
                    Err(ApiError::ResponseError(response.json::<ApiErrorResponse>().await?.message
                        .unwrap_or("errors.auth".to_string())))
                }
            }
            Err(e) => Err(ApiError::from(e))
        }
    }

    async fn credentials(
        &self,
    ) -> Result<AuthModel, ApiError> {
        match self
            .api_client
            .get([&self.api_url, "auth"].join("/"))
            .send()
            .await
        {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    Ok(response.json::<ApiResponse<AuthModel>>().await?.data)
                } else {
                    Err(ApiError::ResponseError(response.json::<ApiErrorResponse>().await?.message
                        .unwrap_or("errors.auth".to_string())))
                }
            }
            Err(e) => Err(ApiError::from(e))
        }
    }
}