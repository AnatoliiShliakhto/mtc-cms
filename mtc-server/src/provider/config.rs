#![allow(dead_code)]
use super::*;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: Cow<'static, str>,
    pub http_port: Cow<'static, str>,
    pub https_port: Cow<'static, str>,
    pub front_end_url: Cow<'static, str>,

    pub password_salt: Cow<'static, str>,

    pub data_path: Cow<'static, str>,
    pub www_path: Cow<'static, str>,
    pub storage_path: Cow<'static, str>,
    pub private_storage_path: Cow<'static, str>,
    pub cert_path: Cow<'static, str>,
    pub log_path: Cow<'static, str>,
    pub db_path: Cow<'static, str>,
    pub migration_path: Cow<'static, str>,

    pub db_namespace: Cow<'static, str>,
    pub db_name: Cow<'static, str>,
    pub session_expiration: i64,

    pub max_body_limit: usize,
    pub rows_per_page: usize,

    pub api_cache_control: Cow<'static, str>,
    pub public_cache_control: Cow<'static, str>,
    pub protected_cache_control: Cow<'static, str>,

    pub strict_transport_security: Cow<'static, str>,
    pub content_security_policy: Cow<'static, str>,
    pub x_content_type_options: Cow<'static, str>,
}

impl Config {
    pub fn init() -> Config {
        dotenv::dotenv().ok();

        let data_path = env!("DATA_PATH");

        Self {
            host: env!("HOST").into(),
            http_port: env!("HTTP_PORT").into(),
            https_port: env!("HTTPS_PORT").into(),
            password_salt: env!("PASSWORD_SALT").into(),
            db_path: [&data_path, "db"].join("/").into(),
            db_namespace: env!("DB_NAMESPACE").into(),
            db_name: env!("DB_NAME").into(),
            session_expiration: env!("SESSION_EXPIRATION_IN_MINUTES")
                .trim()
                .parse::<i64>()
                .unwrap_or(24 * 60),
            front_end_url: env!("FRONT_END_URL").into(),
            max_body_limit: env!("MAX_BODY_LIMIT")
                .trim()
                .parse::<usize>()
                .unwrap_or(104_857_600),
            rows_per_page: env!("ROWS_PER_PAGE")
                .trim()
                .parse::<usize>()
                .unwrap_or(10),
            www_path: if cfg!(debug_assertions) {
                "./target/dx/mtc-wasm/debug/web/public".into()
            } else {
                [&data_path, "www"].join("/").into()
            },
            storage_path: [&data_path, "public"].join("/").into(),
            private_storage_path: [&data_path, "protected"].join("/").into(),
            cert_path: [&data_path, "cert"].join("/").into(),
            log_path: [&data_path, "log"].join("/").into(),
            migration_path: [&data_path, "migrations"].join("/").into(),
            api_cache_control: env!("API_CACHE_CONTROL").into(),
            public_cache_control: env!("PUBLIC_CACHE_CONTROL").into(),
            protected_cache_control: env!("PROTECTED_CACHE_CONTROL").into(),
            strict_transport_security: env!("STRICT_TRANSPORT_SECURITY").into(),
            content_security_policy: env!("CONTENT_SECURITY_POLICY").into(),
            x_content_type_options: env!("X_CONTENT_TYPE_OPTIONS").into(),
            data_path: data_path.into(),
        }
    }

    fn get_env(name: &str) -> Cow<str> {
        std::env::var(name)
            .map_err(|_| error!("ENV VARIABLE missing: {name}"))
            .unwrap()
            .into()
    }
}