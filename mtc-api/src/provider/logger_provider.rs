use tracing::level_filters::LevelFilter;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::provider::config_provider::Config;

pub struct Logger;

impl Logger {
    pub fn init(config: &Config) {
        let env_filter = EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy();

        let file_appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix("mtc-cms.logging")
            .build(&config.log_path)
            .expect("failed to initialize rolling file appender");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        let stdout_layer = tracing_subscriber::fmt::layer().compact();
        let store_layer = tracing_subscriber::fmt::layer()
            .with_writer(non_blocking);

        tracing_subscriber::registry()
            .with(stdout_layer)
            .with(store_layer)
            .with(env_filter)
            .init();
    }
}