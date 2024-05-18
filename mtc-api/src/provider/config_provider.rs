use std::env;

use dotenvy::dotenv;
use tracing::error;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub setup_login: String,
    pub setup_password: String,
    pub db_path: String,
    pub db_namespace: String,
    pub db_name: String,
    pub session_expiration: usize,
    pub front_end_url: String,
    pub password_salt: String,
    pub max_body_limit: usize,
    pub rows_per_page: usize,
}

#[cfg(debug_assertions)]
pub static RUNTIME_STACK_SIZE: usize = 20 * 1024 * 1024; // 20MiB in debug mode
#[cfg(not(debug_assertions))]
pub static RUNTIME_STACK_SIZE: usize = 10 * 1024 * 1024; // 10MiB in release mode

pub static RUNTIME_MAX_BLOCKING_THREADS: usize = 512;

pub const SESSION_USER_KEY: &str = "user";

impl Config {
    pub fn init() -> Config {
        dotenv().ok();

        Self {
            host: get_env("HOST"),
            password_salt: get_env("PASSWORD_SALT"),
            db_path: get_env("DB_PATH"),
            db_namespace: get_env("DB_NAMESPACE"),
            db_name: get_env("DB_NAME"),
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
    }
}

fn get_env(name: &str) -> String {
    env::var(name).map_err(|_| error!("ENV missing: {name}")).unwrap()
}
