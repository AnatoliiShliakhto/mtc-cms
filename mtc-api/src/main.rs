#![forbid(unsafe_code)]

use std::future::Future;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use axum::extract::DefaultBodyLimit;
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_sessions::{ExpiredDeletion, SessionManagerLayer};
use tower_sessions::cookie::Key;
use tower_sessions_surrealdb_store::SurrealSessionStore;
use tracing::error;
use tracing::level_filters::LevelFilter;
use tracing::log::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::provider::config_provider::{Config, RUNTIME_MAX_BLOCKING_THREADS, RUNTIME_STACK_SIZE};
use crate::provider::database_provider::DatabaseProvider;
use crate::provider::redirect_provider::redirect_http_to_https;
use crate::routes::routes;
use crate::state::AppState;

mod state;
mod error;
mod provider;
mod model;
mod repository;
mod service;
mod middleware;
mod routes;
mod handler;

fn main() -> ExitCode {
    match with_enough_stack(app()) {
        Ok(..) => ExitCode::SUCCESS,
        Err(e) => {
            error!(e);
            ExitCode::FAILURE
        }
    }
}

async fn app() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::init();
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("mtc-api")
        .filename_suffix("log")
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

    info!("\x1b[38;5;11m🌟 242 MTC REST API Service 🌟\x1b[0m");

    let db = DatabaseProvider::init(&config).await?;
    info!("\x1b[38;5;6mConnection to the database is successful!\x1b[0m");

    let session_store = SurrealSessionStore::new(db.clone(), "sessions".to_string());
    tokio::task::spawn(session_store.clone().continuously_delete_expired(
        tokio::time::Duration::from_secs(60 * 10),
    ));
    let state = Arc::new(AppState::new(config.clone(), db).await?);

    let session_service = ServiceBuilder::new().layer(
        SessionManagerLayer::new(session_store)
            .with_name("mtc-api.sid")
            .with_private(Key::try_from(config.session_secure_key.as_bytes()).unwrap()));

    tokio::spawn(redirect_http_to_https((
        config.host.clone(),
        config.http_port.clone(),
        config.https_port.clone(),
    )));

    let tls_config = RustlsConfig::from_pem_file(
        PathBuf::from(&config.cert_path)
            .join("cert.pem"),
        PathBuf::from(&config.cert_path)
            .join("key.pem"),
    )
        .await?;

    let app = Router::new()
        .nest("/api", routes(state))
        .nest_service("/", ServeDir::new(&config.public_path))
        .layer(session_service)
        .layer(DefaultBodyLimit::max(config.max_body_limit));


    info!("\x1b[38;5;6mServer started successfully at \x1b[38;5;13m{0}:{1}\x1b[0m -> https://{2}:{3}",
        &config.host, &config.https_port, &config.front_end_url, &config.https_port);

    let https_host: SocketAddr = format!("{}:{}", &config.host, &config.https_port)
        .parse().expect("Unable to parse socket address");
    axum_server::bind_rustls(https_host, tls_config)
        .serve(app.into_make_service()).await?;

    Ok(())
}

fn with_enough_stack<T>(fut: impl Future<Output=T> + Send) -> T {
    // Start a Tokio runtime with custom configuration for embedded SurrealDB
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(RUNTIME_MAX_BLOCKING_THREADS)
        .thread_stack_size(RUNTIME_STACK_SIZE)
        .thread_name("mtc-api-worker")
        .build()
        .unwrap()
        .block_on(fut)
}
