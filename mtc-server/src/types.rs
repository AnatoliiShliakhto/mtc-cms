pub type Result<T> = std::result::Result<T, crate::Error>;
#[cfg(debug_assertions)]
pub type Session = axum_session::Session<axum_session_surreal::SessionSurrealPool<surrealdb::engine::remote::ws::Client>>;
#[cfg(debug_assertions)]
pub type Database = surrealdb::Surreal<surrealdb::engine::remote::ws::Client>;
#[cfg(not(debug_assertions))]
pub type Session = axum_session::Session<axum_session_surreal::SessionSurrealPool<surrealdb::engine::local::Db>>;
#[cfg(not(debug_assertions))]
pub type Database = surrealdb::Surreal<surrealdb::engine::local::Db>;