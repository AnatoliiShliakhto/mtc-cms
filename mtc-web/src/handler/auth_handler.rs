use mtc_model::auth_model::{AuthModel, SignInModel};

use crate::error::api_error::ApiError;
use crate::handler::{ApiHandler, HandlerResponse};

pub trait AuthHandler {
    async fn sign_in(&self, login: String, password: String) -> Result<AuthModel, ApiError>;
    async fn sign_out(&self) -> Result<AuthModel, ApiError>;
    async fn get_credentials(&self) -> Result<AuthModel, ApiError>;
}

impl AuthHandler for ApiHandler {
    async fn sign_in(
        &self,
        login: String,
        password: String,
    ) -> Result<AuthModel, ApiError> {
        self
            .api_client
            .post([&self.api_url, "auth"].join("/"))
            .json(&SignInModel { login: login.to_string(), password: password.to_string() })
            .send()
            .await
            .consume_data()
            .await
    }

    async fn sign_out(
        &self,
    ) -> Result<AuthModel, ApiError> {
        self
            .api_client
            .delete([&self.api_url, "auth"].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }

    async fn get_credentials(
        &self,
    ) -> Result<AuthModel, ApiError> {
        self
            .api_client
            .get([&self.api_url, "auth"].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }
}