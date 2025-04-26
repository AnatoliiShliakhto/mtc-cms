use super::*;

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub security: SecurityConfig,
    pub paths: PathConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub pagination: PaginationConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: Cow<'static, str>,
    pub http_port: Cow<'static, str>,
    pub https_port: Cow<'static, str>,
    pub front_end_url: Cow<'static, str>,
}

#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub password_salt: Cow<'static, str>,
    pub strict_transport_security: Cow<'static, str>,
    pub content_security_policy: Cow<'static, str>,
    pub x_content_type_options: Cow<'static, str>,
    pub session_expiration: i64,
    pub max_body_limit: usize,
}

#[derive(Debug, Clone)]
pub struct PathConfig {
    pub data_path: Cow<'static, str>,
    pub www_path: Cow<'static, str>,
    pub storage_path: Cow<'static, str>,
    pub private_storage_path: Cow<'static, str>,
    pub cert_path: Cow<'static, str>,
    pub log_path: Cow<'static, str>,
    pub migration_path: Cow<'static, str>,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub db_path: Cow<'static, str>,
    pub db_namespace: Cow<'static, str>,
    pub db_name: Cow<'static, str>,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub api_cache_control: Cow<'static, str>,
    pub public_cache_control: Cow<'static, str>,
    pub protected_cache_control: Cow<'static, str>,
}

#[derive(Debug, Clone)]
pub struct PaginationConfig {
    pub rows_per_page: usize,
}

impl Config {
    pub fn init() -> Config {
        dotenv::dotenv().ok();
        let data_path = env!("DATA_PATH");

        Config {
            server: ServerConfig {
                host: env!("HOST").into(),
                http_port: env!("HTTP_PORT").into(),
                https_port: env!("HTTPS_PORT").into(),
                front_end_url: env!("FRONT_END_URL").into(),
            },
            security: SecurityConfig {
                password_salt: env!("PASSWORD_SALT").into(),
                strict_transport_security: env!("STRICT_TRANSPORT_SECURITY").into(),
                content_security_policy: env!("CONTENT_SECURITY_POLICY").into(),
                x_content_type_options: env!("X_CONTENT_TYPE_OPTIONS").into(),
                session_expiration: env!("SESSION_EXPIRATION_IN_MINUTES").parse().unwrap(),
                max_body_limit: env!("MAX_BODY_LIMIT").parse().unwrap(),
            },
            paths: Self::init_paths(&data_path),
            database: DatabaseConfig {
                db_path: Self::build_path(&data_path, "db"),
                db_namespace: env!("DB_NAMESPACE").into(),
                db_name: env!("DB_NAME").into(),
            },
            cache: CacheConfig {
                api_cache_control: env!("API_CACHE_CONTROL").into(),
                public_cache_control: env!("PUBLIC_CACHE_CONTROL").into(),
                protected_cache_control: env!("PROTECTED_CACHE_CONTROL").into(),
            },
            pagination: PaginationConfig {
                rows_per_page: env!("ROWS_PER_PAGE").parse().unwrap(),
            },
        }
    }

    fn init_paths(data_path: &str) -> PathConfig {
        PathConfig {
            www_path: if cfg!(debug_assertions) {
                "./target/dx/mtc-wasm/debug/web/public".into()
            } else {
                Self::build_path(data_path, "www")
            },
            storage_path: Self::build_path(data_path, "public"),
            private_storage_path: Self::build_path(data_path, "protected"),
            cert_path: Self::build_path(data_path, "cert"),
            log_path: Self::build_path(data_path, "log"),
            migration_path: Self::build_path(data_path, "migrations"),
            data_path: Cow::Owned(data_path.to_string()),
        }
    }

    fn build_path(base: &str, path: &str) -> Cow<'static, str> {
        [base, path].join("/").into()
    }
}
