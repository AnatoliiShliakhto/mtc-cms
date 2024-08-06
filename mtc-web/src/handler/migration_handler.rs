use mtc_model::auth_model::SignInModel;
use mtc_model::list_model::StringListModel;
use crate::error::api_error::ApiError;
use crate::handler::{ApiHandler, HandlerNullResponse, HandlerResponse};

pub trait MigrationHandler {
    async fn get_migrations(&self) -> Result<StringListModel, ApiError>;
    async fn migrate(&self, login: String, password: String) -> Result<(), ApiError>;
}

impl MigrationHandler for ApiHandler {
    async fn get_migrations(&self) -> Result<StringListModel, ApiError> {
        self.api_client
            .get([&self.api_url, "migration"].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }

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
