use crate::prelude::*;
use database::*;

mod config;
mod database;
mod logger;
mod template;

pub(crate) mod prelude {
    pub(crate) use super::{config::*, logger::*, template::*, Provider};
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
    /// Initializes the template using the provided configuration.
    pub async fn template_init(config: &Config) -> Template {
        Template::init(config).await
    }
}
