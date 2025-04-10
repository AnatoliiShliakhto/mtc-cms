use crate::prelude::*;
use database::*;

mod config;
mod logger;
mod database;


pub(crate) mod prelude {
    pub(crate) use super::{
        Provider,
        config::*,
        logger::*,
    };
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
}