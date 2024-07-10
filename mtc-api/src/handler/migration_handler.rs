use std::sync::Arc;

use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use axum::extract::State;
use tower_sessions::Session;
use tracing::info;

use mtc_model::auth_model::SignInModel;

use crate::handler::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::request_model::ValidatedPayload;
use crate::model::response_model::ApiResponse;
use crate::service::migration_service::MigrationTrait;
use crate::state::AppState;

pub async fn migration_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<SignInModel>,
) -> Result<()> {
    if !state.migration_service.is_new().await? {
        session.permission("administrator").await?;
    }

    let sql = state.migration_service.get_migration().await?;
    if sql.is_none() {
        return Ok(ApiResponse::Ok);
    }

    let salt = SaltString::from_b64(&state.cfg.password_salt).unwrap();

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .expect("Error occurred while encrypted password")
        .to_string();

    let responses = state
        .db
        .query(sql.unwrap())
        .bind(("login", payload.login.trim().to_uppercase()))
        .bind(("password", password_hash.as_str()))
        .await?;

    info!("{responses:#?}\nMigration done!");

    Ok(ApiResponse::Ok)
}
