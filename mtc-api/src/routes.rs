use std::sync::Arc;

use axum::http::header::{CONTENT_TYPE, COOKIE};
use axum::http::HeaderValue;
use axum::middleware::from_fn_with_state;
use axum::Router;
use axum::routing::{get, post};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::log::info;

use crate::handler::api_handler::*;
use crate::handler::auth_handler::*;
use crate::handler::group_handler::*;
use crate::handler::health_handler::*;
use crate::handler::permissions_handler::*;
use crate::handler::role_handler::*;
use crate::handler::schema_handler::*;
use crate::handler::setup_handler::*;
use crate::handler::user_handler::*;
use crate::middleware::auth_middleware::middleware_auth_handler;
use crate::state::AppState;

pub fn routes(
    state: Arc<AppState>
) -> Router {
    let front_end_url = &state.cfg.front_end_url;

    info!("\x1b[38;5;6mFront end CORS allowed URL: \x1b[38;5;13m{front_end_url}\x1b[0m");
    let cors_layer = CorsLayer::new()
        .allow_origin(front_end_url.parse::<HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE, COOKIE])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::DELETE,
        ]);

    Router::new()
        //todo: universal custom api handlers
        .route("/:api/:slug", get(api_get_collection_item_handler))
        .route("/:api", get(api_get_single_handler).post(api_create_handler))

        .route("/schema/:slug/fields", get(schema_get_fields_handler).post(schema_update_fields_handler))
        .route("/schema/:slug", get(schema_get_handler).post(schema_update_handler).delete(schema_delete_handler))
        .route("/schema", post(schema_create_handler))
        .route("/schemas/:page", get(schema_list_handler))
        .route("/schemas", get(schema_list_handler).post(schema_create_handler))

        //todo: additional user fields
        .route("/user/:login/groups", get(user_get_groups_handler).post(user_set_groups_handler))
        .route("/user/:login/permissions", get(user_get_permissions_handler))
        .route("/user/:login/roles", get(user_get_roles_handler).post(user_set_roles_handler))
        .route("/user/:login", get(user_get_handler).post(user_update_handler).delete(user_delete_handler))
        .route("/user", post(user_create_handler))
        .route("/users/:page", get(user_list_handler))
        .route("/users", get(user_list_handler).post(user_create_handler))

        .route("/group/:slug", get(group_get_handler).post(group_update_handler).delete(group_delete_handler))
        .route("/group", post(group_create_handler))
        .route("/groups/:page", get(group_list_handler))
        .route("/groups", get(group_list_handler).post(group_create_handler))

        .route("/role/:slug/permissions", get(role_get_permissions).post(role_set_permissions))
        .route("/role/:slug", get(role_get_handler).post(role_update_handler).delete(role_delete_handler))
        .route("/role", post(role_create_handler))
        .route("/roles/:page", get(role_list_handler))
        .route("/roles", get(role_list_handler).post(role_create_handler))

        .route("/permissions", get(permissions_list_handler))

        //todo: change password
        .route("/auth", get(get_credentials_handler).post(sign_in_handler).delete(sign_out_handler))

        .route("/setup", post(setup_handler))
        .route("/health", get(health_handler))

        .layer(ServiceBuilder::new().layer(from_fn_with_state(Arc::clone(&state), middleware_auth_handler)))
        .with_state(state)
        .layer(cors_layer)
}