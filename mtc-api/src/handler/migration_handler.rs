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
        session.permission("schema::write").await?;
    }

    let salt = SaltString::from_b64(&state.cfg.password_salt).unwrap();

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .expect("Error occurred while encrypted password")
        .to_string();
    
    while let Some(file_name) = state.migration_service.get_migration_file_name().await? {
        let sql = state.migration_service.get_migration(&file_name).await?;

        state
            .db
            .query(sql)
            .bind(("login", payload.login.trim().to_uppercase()))
            .bind(("password", password_hash.as_str()))
            .await?;
        info!("Migration {} is done!", file_name);

        state.migration_service.delete_migration_file(&file_name).await?;
    } 
    
    Ok(ApiResponse::Ok)
}
