use crate::prelude::*;
use crate::provider::smtp_client::SmtpClient;
use database::*;

mod config;
mod database;
mod logger;
mod smtp_client;
mod template;

pub(crate) mod prelude {
    pub(crate) use super::{config::*, logger::*, smtp_client::*, template::*, Provider};
}

pub(crate) struct Provider;

impl Provider {
    /// Initializes a new [`Config`] instance with default values.
    pub fn config_init() -> Config {
        Config::init()
    }
    /// Initializes the database connection using the provided configuration.
    pub async fn database_init(config: &Config) -> Database {
        database_init(config).await.unwrap()
    }
    /// Initializes the smtp client using the provided configuration.
    pub async fn smtp_client_init(config: &Config) -> SmtpClient {
        SmtpClient::init(config)
    }
    /// Initializes the template using the provided configuration.
    pub async fn template_init(config: &Config) -> Template {
        Template::init(config).await
    }
}
