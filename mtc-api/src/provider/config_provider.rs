use std::env;

use dotenvy::dotenv;
use once_cell::sync::Lazy;
use tracing::error;

#[derive(Debug, Clone)]
pub struct ConfigProvider {
    pub host: String,
    pub setup_login: String,
    pub setup_password: String,
    pub db_url: String,
    pub db_namespace: String,
    pub db_name: String,
    pub session_secret_key: String,
    pub session_expiration: usize,
    pub front_end_url: String,
    pub password_salt: String,
    pub max_body_limit: usize,
    pub rows_per_page: usize,
}

// 2 MiB default stack size is not enough for deep queries
pub static RUNTIME_STACK_SIZE: Lazy<usize> = Lazy::new(|| {
    if cfg!(debug_assertions) {
        20 * 1024 * 1024 // 20MiB in debug mode
    } else {
        10 * 1024 * 1024 // 10MiB in release mode
    }
});

pub static RUNTIME_MAX_BLOCKING_THREADS: Lazy<usize> = Lazy::new(|| 512);

pub const SESSION_USER_KEY: &str = "user";

pub static CFG: Lazy<ConfigProvider> = Lazy::new(|| {
    dotenv().ok();
    ConfigProvider {
        host: get_env("HOST"),
        password_salt: get_env("PASSWORD_SALT"),
        db_url: get_env("DB_URL"),
        db_namespace: get_env("DB_NAMESPACE"),
        db_name: get_env("DB_NAME"),
        session_secret_key: get_env("SESSION_SECRET_KEY"),
        session_expiration: get_env("SESSION_EXPIRATION_IN_MINUTES")
            .trim().parse::<usize>().unwrap_or(30),
        front_end_url: get_env("FRONT_END_URL"),
        max_body_limit: get_env("MAX_BODY_LIMIT")
            .trim().parse::<usize>().unwrap_or(104_857_600),
        rows_per_page: get_env("ROWS_PER_PAGE")
            .trim().parse::<usize>().unwrap_or(10),
        setup_login: get_env("SETUP_ADMIN_LOGIN"),
        setup_password: get_env("SETUP_ADMIN_PASSWORD"),
    }
});

fn get_env(name: &str) -> String {
    env::var(name).map_err(|_| error!("ENV missing: {name}")).unwrap()
}
