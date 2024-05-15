#![forbid(unsafe_code)]

use std::future::Future;
use std::process::ExitCode;
use std::sync::Arc;

use axum::extract::DefaultBodyLimit;
use axum::Router;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer};
use tower_sessions::cookie::time::Duration;
use tower_sessions_surrealdb_store::SurrealSessionStore;
use tracing::error;
use tracing::log::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::provider::config_provider::{CFG, RUNTIME_MAX_BLOCKING_THREADS, RUNTIME_STACK_SIZE};
use crate::provider::database_provider::{DB, db_init};
use crate::routes::routes;
use crate::state::AppState;

mod state;
mod error;
mod provider;
mod model;
mod repository;
mod paginator;
mod service;
mod middleware;
pub mod routes;
pub mod handler;

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
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("mtc-cms.logging")
        .build("./log/mtc-api")
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

    db_init().await;
    let state = Arc::new(AppState::new().await?);
    info!("\x1b[38;5;6mConnection to the database is successful!\x1b[0m");

    let session_store = SurrealSessionStore::new(DB.clone(), "sessions".to_string());

    tokio::task::spawn(session_store.clone().continuously_delete_expired(
        tokio::time::Duration::from_secs(60 * 10),
    ));

    let session_service = ServiceBuilder::new().layer(
        SessionManagerLayer::new(session_store)
            .with_name("mtc-api.sid")
            .with_secure(true)
            .with_expiry(Expiry::OnInactivity(Duration::minutes(CFG.session_expiration as i64))),
    );

    let app = Router::new()
        .nest("/api", routes(state))
        .nest_service("/", ServeDir::new("public"))
        .layer(session_service)
        .layer(DefaultBodyLimit::max(CFG.max_body_limit));


    let listener = TcpListener::bind(&CFG.host).await?;
    info!("\x1b[38;5;6mServer started successfully at \x1b[38;5;13m{}\x1b[0m", &CFG.host);

    //todo: add HTTPS with rustls/axum_server
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

fn with_enough_stack<T>(fut: impl Future<Output=T> + Send) -> T {
    // Start a Tokio runtime with custom configuration for embedded SurrealDB
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(*RUNTIME_MAX_BLOCKING_THREADS)
        .thread_stack_size(*RUNTIME_STACK_SIZE)
        .thread_name("mtc-api-worker")
        .build()
        .unwrap()
        .block_on(fut)
}