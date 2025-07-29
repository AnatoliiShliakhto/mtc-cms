use prelude::*;

mod provider;
mod error;
mod state;
mod routes;
mod middleware;
mod handlers;
mod repository;
mod types;
mod models;

pub(crate) mod prelude {
    pub(crate) use {
        ::mtc_common::prelude::*,
        ::server_macros::handler,

        std::{
            net::SocketAddr, path::PathBuf, sync::Arc,
            collections::BTreeSet, fmt::Write,
        },

        serde::{de::DeserializeOwned, Deserialize, Serialize},
        serde_json::{json, Map},

        axum::{
            extract::{
                DefaultBodyLimit, rejection::{FormRejection, JsonRejection},
                FromRequest, Request, State, Path, Multipart, Query,
            },
            http::{
                header::{
                    CACHE_CONTROL, STRICT_TRANSPORT_SECURITY, X_FRAME_OPTIONS,
                    X_CONTENT_TYPE_OPTIONS, CONTENT_TYPE, ACCEPT_ENCODING, CONTENT_SECURITY_POLICY,
                },
                HeaderValue, HeaderName, status::StatusCode},
            middleware::{Next, from_fn}, Router, Form, Json,
            response::{Response, IntoResponse, Redirect},
            routing::{get, post, delete},
        },
        axum_server::tls_rustls::RustlsConfig,

        axum_session::{SessionConfig, SessionLayer, SessionMode, SessionStore},
        axum_session_surreal::SessionSurrealPool,

        tower::ServiceBuilder,
        tower_http::{
            compression::CompressionLayer, cors::CorsLayer,
            services::{ServeDir},
            set_header::SetResponseHeaderLayer,
        },

        argon2::{Argon2, password_hash::SaltString, PasswordHasher, PasswordVerifier, PasswordHash},
        tracing::log::{error, warn, info},
        tracing_appender::{
            rolling::{RollingFileAppender, Rotation},
            non_blocking::WorkerGuard,
        },
        tracing_subscriber::{
            filter::LevelFilter, fmt::layer, layer::SubscriberExt,
            util::SubscriberInitExt, EnvFilter,
        },
        tokio::fs,
        rand::Rng,
        chrono::Duration,
        magic_crypt::{MagicCryptTrait, new_magic_crypt},

        super::{
            types::*,
            error::prelude::*,
            state::AppState,
            routes::*,
            models::prelude::*,
            provider::prelude::*,
            repository::prelude::*,
            middleware::prelude::*,
            handlers::prelude::*,
        }
    };
}

#[tokio::main]
async fn main() {
    let config = Provider::config_init();
    let _guard = logger_init(&config.paths.log_path);

    info!("\x1b[38;5;11m🌟 MTC-CMS Server 🌟\x1b[0m");

    //crypto provider init
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Crypto Provide initialization failed.");

    let db = Provider::database_init(&config).await;

    let session_config = SessionConfig::default()
        .with_table_name("sessions")
        .with_lifetime(Duration::minutes(config.security.session_expiration))
        .with_mode(SessionMode::Persistent);

    let session_store =
        SessionStore::new(Some(SessionSurrealPool::new(db.clone())), session_config)
            .await
            .unwrap();

    let template = Provider::template_init(&config).await;

    let state = Arc::new(AppState::init(config, db, template));

    let tls_config = RustlsConfig::from_pem_file(
        PathBuf::from(&*state.config.paths.cert_path).join("ssl.crt"),
        PathBuf::from(&*state.config.paths.cert_path).join("private.key"),
    ).await.unwrap();

    let compression_layer: CompressionLayer = CompressionLayer::new()
        .br(true)
        .gzip(true)
        .zstd(true);

    let static_headers = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_str(&state.config.cache.public_cache_control).unwrap())
        )
        .layer(SetResponseHeaderLayer::if_not_present(
            STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_str(&state.config.security.strict_transport_security).unwrap())
        )
        .layer(SetResponseHeaderLayer::if_not_present(
            X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_str(&state.config.security.x_content_type_options).unwrap())
        )
        .layer(SetResponseHeaderLayer::if_not_present(
            X_FRAME_OPTIONS,
            HeaderValue::from_str(&state.config.security.x_frame_options).unwrap())
        );

    let cors_layer = CorsLayer::new()
            .allow_origin([state.config.server.front_end_url.parse().unwrap()])
            .allow_headers([CONTENT_TYPE, ACCEPT_ENCODING,
                HeaderName::from_static("session"),
            ])
            .allow_methods([
                axum::http::Method::GET,
                axum::http::Method::POST,
                axum::http::Method::PATCH,
                axum::http::Method::DELETE,
                axum::http::Method::OPTIONS
            ])
        .allow_credentials(true);

    let app = Router::new()
        .nest_service(PRIVATE_ASSETS_PATH, ServeDir::new(&*state.config.paths.private_storage_path))
        .layer(from_fn(middleware_protected_storage_handler))
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_str(&state.config.cache.protected_cache_control).unwrap())
        )
        .nest(API_PATH, routes(state.clone()))
        .layer(SessionLayer::new(session_store))
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_str(&state.config.cache.api_cache_control).unwrap())
        )
        .layer(from_fn(middleware_headers_check_handler))
        .fallback(Redirect::permanent("/"))
        .nest_service(PUBLIC_ASSETS_PATH, ServeDir::new(&*state.config.paths.storage_path))
        .route("/service_worker", get(service_worker_handler))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", state.config.paths.www_path))
        )
        .nest_service(
            "/wasm",
            ServeDir::new(format!("{}/wasm", state.config.paths.www_path))
        )
        .route(
            "/",
            get({
                let state = state.clone();
                move || async move { get_index_html(state).await }
            }),
        )
        .layer(compression_layer)
        .layer(static_headers)
        .layer(cors_layer)
        .layer(DefaultBodyLimit::max(state.config.security.max_body_limit));

    info!("\x1b[38;5;6mServer started successfully at \x1b[38;5;13m{0}:{1}\x1b[0m -> {2}:{3}",
        &state.config.server.host, &state.config.server.https_port,
        &state.config.server.front_end_url, &state.config.server.https_port);

    let https_host: SocketAddr = format!("{}:{}", &state.config.server.host, &state.config.server.https_port)
        .parse().expect("Unable to parse socket address");

    // run AXUM server with TLS
    axum_server::bind_rustls(https_host, tls_config)
        .serve(app.into_make_service()).await.unwrap();
}