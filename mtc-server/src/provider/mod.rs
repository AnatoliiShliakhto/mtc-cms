use crate::prelude::*;
use database::*;
use logger::*;

mod config;
mod logger;
mod database;


pub mod prelude {
    pub use super::{
        Provider,
        config::*,
    };
}

pub struct Provider;

impl Provider {
    pub fn config_init() -> Config {
        let config = Config::init();
        logger_init(&config.log_path);
        config
    }
    pub async fn database_init(config: &Config) -> Database {
        database_init(config).await.unwrap()
    }
}