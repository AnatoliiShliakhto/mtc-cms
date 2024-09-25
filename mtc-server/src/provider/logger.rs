use super::*;

pub fn logger_init(log_path: &str) {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("mtc-srv")
        .filename_suffix("log")
        .max_log_files(10)
        .build(log_path)
        .expect("failed to initialize rolling file appender");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let stdout_layer = layer().compact();
    let store_layer = layer().with_writer(non_blocking);

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(store_layer)
        .with(env_filter)
        .init();
}