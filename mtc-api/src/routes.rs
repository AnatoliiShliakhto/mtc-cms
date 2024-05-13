use std::sync::Arc;

use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::HeaderValue;
use axum::middleware::from_fn_with_state;
use axum::Router;
use axum::routing::{get, post};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::log::info;

use crate::handler::health_handler::*;
use crate::handler::permissions_handler::*;
use crate::handler::role_handler::*;
use crate::handler::setup_handler::*;
use crate::handler::user_handler::*;
use crate::middleware::auth_middleware::auth_handler;
use crate::provider::config_provider::CFG;
use crate::state::AppState;

pub fn routes(state: Arc<AppState>) -> Router {
    let front_end_url = &CFG.front_end_url;

    info!("\x1b[38;5;6mFront end CORS allowed URL: \x1b[38;5;13m{front_end_url}\x1b[0m");
    let cors_layer = CorsLayer::new()
        .allow_origin(front_end_url.parse::<HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::DELETE,
        ]);

    Router::new()
        .route("/permissions/:id", get(permissions_role_handler))
        .route("/permissions", get(permissions_list_handler))
        .route("/user/:id", get(user_get_handler))
        .route("/user", get(user_list_handler))
        .route("/role/:id", get(role_get_handler).post(role_update_handler).delete(role_delete_handler))
        .route("/role", get(role_list_handler).post(role_create_handler))
        .route("/setup", post(setup_handler))
        .layer(ServiceBuilder::new().layer(from_fn_with_state(
            Arc::clone(&state),
            auth_handler,
        )))
        .with_state(state)
        .route("/health", get(health_handler))
        .layer(cors_layer)
}