use mtc_model::auth_model::SignInModel;

use crate::error::api_error::ApiError;
use crate::handler::{ApiHandler, HandlerNullResponse};

pub trait MigrationHandler {
    async fn migrate(&self, login: String, password: String) -> Result<(), ApiError>;
}

impl MigrationHandler for ApiHandler {
    async fn migrate(&self, login: String, password: String) -> Result<(), ApiError> {
        self.api_client
            .post([&self.api_url, "migration"].join("/"))
            .json(&SignInModel {
                login: login.to_string(),
                password: password.to_string(),
            })
            .send()
            .await
            .consume()
            .await
    }
}
