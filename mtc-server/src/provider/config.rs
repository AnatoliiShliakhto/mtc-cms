#![allow(dead_code)]
use super::*;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: Cow<'static, str>,
    pub http_port: Cow<'static, str>,
    pub https_port: Cow<'static, str>,
    pub front_end_url: Cow<'static, str>,

    pub password_salt: Cow<'static, str>,

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
    pub session_secure_key: Cow<'static, str>,

    pub max_body_limit: usize,
    pub rows_per_page: usize,

    pub api_cache_control: Cow<'static, str>,
    pub public_cache_control: Cow<'static, str>,
    pub protected_cache_control: Cow<'static, str>,

    pub strict_transport_security: Cow<'static, str>,
    pub content_security_policy: Cow<'static, str>,
    pub x_content_type_options: Cow<'static, str>,
    pub x_frame_options: Cow<'static, str>,
}

impl Config {
    pub fn init() -> Config {
        dotenv::dotenv().ok();

        let data_path = Self::get_env("DATA_PATH");

        Self {
            host: Self::get_env("HOST"),
            http_port: Self::get_env("HTTP_PORT"),
            https_port: Self::get_env("HTTPS_PORT"),
            password_salt: Self::get_env("PASSWORD_SALT"),
            db_path: [&data_path, "db"].join("/").into(),
            db_namespace: Self::get_env("DB_NAMESPACE"),
            db_name: Self::get_env("DB_NAME"),
            session_expiration: Self::get_env("SESSION_EXPIRATION_IN_MINUTES")
                .trim()
                .parse::<i64>()
                .unwrap_or(24 * 60),
            session_secure_key: Self::get_env("SESSION_SECURE_KEY"),
            front_end_url: Self::get_env("FRONT_END_URL"),
            max_body_limit: Self::get_env("MAX_BODY_LIMIT")
                .trim()
                .parse::<usize>()
                .unwrap_or(104_857_600),
            rows_per_page: Self::get_env("ROWS_PER_PAGE")
                .trim()
                .parse::<usize>()
                .unwrap_or(10),
            www_path: [&data_path, "www"].join("/").into(),
            storage_path: [&data_path, "public"].join("/").into(),
            private_storage_path: [&data_path, "protected"].join("/").into(),
            cert_path: [&data_path, "cert"].join("/").into(),
            log_path: [&data_path, "log"].join("/").into(),
            migration_path: [&data_path, "migrations"].join("/").into(),
            api_cache_control: Self::get_env("API_CACHE_CONTROL"),
            public_cache_control: Self::get_env("PUBLIC_CACHE_CONTROL"),
            protected_cache_control: Self::get_env("PROTECTED_CACHE_CONTROL"),
            strict_transport_security: Self::get_env("STRICT_TRANSPORT_SECURITY"),
            content_security_policy: Self::get_env("CONTENT_SECURITY_POLICY"),
            x_content_type_options: Self::get_env("X_CONTENT_TYPE_OPTIONS"),
            x_frame_options: Self::get_env("X_FRAME_OPTIONS"),
        }
    }

    fn get_env(name: &str) -> Cow<str> {
        std::env::var(name)
            .map_err(|_| error!("ENV VARIABLE missing: {name}"))
            .unwrap()
            .into()
    }
}