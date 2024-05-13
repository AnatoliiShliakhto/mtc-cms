use std::sync::Arc;

use axum::extract::DefaultBodyLimit;
use axum::Router;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer};
use tower_sessions::cookie::time::Duration;
use tower_sessions_surrealdb_store::SurrealSessionStore;
use tracing::log::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::provider::config_provider::CFG;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
            .with_secure(false)
            .with_path("/mtc/api")
            .with_expiry(Expiry::OnInactivity(Duration::minutes(CFG.session_expiration as i64))),
    );

    let static_routing_service = ServeDir::new("public");

    let app = Router::new()
        .nest("/api", routes(state))
//        .merge(routes(state))
        .nest_service("/", static_routing_service)
        .layer(session_service)
        .layer(DefaultBodyLimit::max(CFG.max_body_limit));


    let listener = TcpListener::bind(&CFG.host).await?;
    info!("\x1b[38;5;6mServer started successfully at \x1b[38;5;13m{}\x1b[0m", &CFG.host);

    //todo: add HTTPS with rustls/axum_server
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
