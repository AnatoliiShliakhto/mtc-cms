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

pub mod prelude {
    pub use {
        mtc_model::prelude::*,
        std::{
            future::Future, net::SocketAddr, path::PathBuf, sync::Arc,
            collections::BTreeSet,
        },

        serde::{de::DeserializeOwned, Deserialize, Serialize},
        serde_json::{json, Map},

        axum::{
            async_trait,
            extract::{
                DefaultBodyLimit, rejection::{FormRejection, JsonRejection},
                FromRequest, Request, State, Path, Multipart,
            },
            http::{
                header::{
                    CACHE_CONTROL, STRICT_TRANSPORT_SECURITY,
                    X_CONTENT_TYPE_OPTIONS, X_FRAME_OPTIONS, CONTENT_SECURITY_POLICY,
                    CONTENT_TYPE, COOKIE, ACCEPT_ENCODING,
                },
                HeaderValue, status::StatusCode},
            middleware::Next, Router, Form, Json,
            response::{Response, IntoResponse},
            routing::{get, post, delete},
        },
        axum_server::tls_rustls::RustlsConfig,
        /*
        axum_session::{Key, SameSite, SessionConfig, SessionLayer, SessionMode, SessionStore},
        axum_session_surreal::SessionSurrealPool,
        */
        tower::ServiceBuilder,
        tower_http::{
            compression::CompressionLayer, cors::CorsLayer,
            services::{ServeDir, ServeFile},
            set_header::SetResponseHeaderLayer,
        },
        //chrono::Duration,


        tower_sessions::{
            cookie::{time::Duration, SameSite},
            ExpiredDeletion, Expiry, Session, SessionManagerLayer,
        },
        tower_sessions_surrealdb_store::SurrealSessionStore,


        argon2::{Argon2, password_hash::SaltString, PasswordHasher, PasswordVerifier, PasswordHash},
        tracing::log::{error, info},
        tracing_appender::rolling::{RollingFileAppender, Rotation},
        tracing_subscriber::{
            filter::LevelFilter, fmt::layer, layer::SubscriberExt,
            util::SubscriberInitExt, EnvFilter,
        },
        tokio::fs,

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
    info!("\x1b[38;5;11m🌟 MTC-CMS Server 🌟\x1b[0m");

    //crypto provider init
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Crypto Provide initialization failed.");

    let db = Provider::database_init(&config).await;

    let session_store =
        SurrealSessionStore::new(db.clone(), "sessions".to_string());
    tokio::task::spawn(session_store.clone().continuously_delete_expired(
        tokio::time::Duration::from_secs(60 * 10),
    ));
    let session_service = ServiceBuilder::new().layer(
        SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_same_site(SameSite::Lax)
            .with_name("mtc-api.sid")
            .with_expiry(Expiry::OnInactivity(Duration::minutes(config.session_expiration))),
    );

    /* axum sessions
    let session_config = SessionConfig::default()
        .with_table_name("sessions")
        .with_secure(true)
        .with_session_name("mtc-api.sid")
        .with_max_age(Some(Duration::minutes(config.session_expiration)))
        .with_lifetime(Duration::minutes(config.session_expiration))
        .with_cookie_same_site(SameSite::Lax)
        .with_http_only(true)
        .with_hashed_user_agent(false)
        .with_hashed_ip(false)
        .with_mode(SessionMode::Persistent)
        .with_key(Key::from(config.session_secure_key.as_bytes()));

    let session_store =
        SessionStore::new(Some(SessionSurrealPool::new(db.clone())), session_config).await.unwrap();
    */

    let state = Arc::new(AppState::init(config, db));

    let tls_config = RustlsConfig::from_pem_file(
        PathBuf::from(&*state.config.cert_path).join("ssl.crt"),
        PathBuf::from(&*state.config.cert_path).join("private.key"),
    ).await.unwrap();

    let fallback_service =
        ServeDir::new(&*state.config.www_path)
            .not_found_service(ServeFile::new([&state.config.www_path, "index.html"].join("/")));

    let comression_layer: CompressionLayer = CompressionLayer::new()
        .br(true)
        .gzip(true)
        .zstd(true);
    let public_storage_service = ServiceBuilder::new()
        .service(ServeDir::new(&*state.config.storage_path));

    let protected_storage_service = ServiceBuilder::new()
        .service(ServeDir::new(&*state.config.private_storage_path));

    let static_headers = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_str(&state.config.public_cache_control).unwrap())
        )
        .layer(SetResponseHeaderLayer::if_not_present(
            STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_str(&state.config.strict_transport_security).unwrap())
        )
        .layer(SetResponseHeaderLayer::if_not_present(
            X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_str(&state.config.x_content_type_options).unwrap())
        )
        .layer(SetResponseHeaderLayer::if_not_present(
            X_FRAME_OPTIONS,
            HeaderValue::from_str(&state.config.x_frame_options).unwrap())
        )
        .layer(SetResponseHeaderLayer::if_not_present(
            CONTENT_SECURITY_POLICY,
            HeaderValue::from_str(&state.config.content_security_policy).unwrap())
        );

    let cors_layer = CorsLayer::new()
        .allow_origin(state.config.front_end_url.parse::<HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE, COOKIE, ACCEPT_ENCODING, CACHE_CONTROL])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PATCH,
            axum::http::Method::DELETE,
        ]);

    let app = Router::new()
        .nest_service(PRIVATE_ASSETS_PATH, protected_storage_service)
        //.layer(from_fn(middleware_protected_storage_handler))
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_str(&state.config.protected_cache_control).unwrap())
        )
        .nest(API_PATH, routes(state.clone()))
        .layer(session_service)
        //.layer(SessionLayer::new(session_store))
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_str(&state.config.api_cache_control).unwrap())
        )
        .nest_service(PUBLIC_ASSETS_PATH, public_storage_service)
        .fallback_service(fallback_service)
        .layer(comression_layer)
        .layer(static_headers)
        .layer(cors_layer)
        .layer(DefaultBodyLimit::max(state.config.max_body_limit));

    info!("\x1b[38;5;6mServer started successfully at \x1b[38;5;13m{0}:{1}\x1b[0m -> https://{2}:{3}",
        &state.config.host, &state.config.https_port,
        &state.config.front_end_url, &state.config.https_port);

    let https_host: SocketAddr = format!("{}:{}", &state.config.host, &state.config.https_port)
        .parse().expect("Unable to parse socket address");

    // run AXUM server with TLS
    axum_server::bind_rustls(https_host, tls_config)
        .serve(app.into_make_service()).await.unwrap();
}