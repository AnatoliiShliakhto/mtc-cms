use std::collections::BTreeSet;
use std::sync::Arc;

use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use axum::extract::State;
use tower_sessions::Session;
use tracing::{info};

use mtc_model::auth_model::SignInModel;
use mtc_model::list_model::StringListModel;
use crate::handler::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::request_model::ValidatedPayload;
use crate::model::response_model::{ApiResponse, HandlerResult};
use crate::repository::system_repository::SystemRepositoryTrait;
use crate::service::migration_service::MigrationTrait;
use crate::state::AppState;

pub async fn migration_handler(
    state: State<Arc<AppState>>,
    session: Session,
    ValidatedPayload(payload): ValidatedPayload<SignInModel>,
) -> Result<()> {
    let mut migrations = state.system_service.get_migrations().await?;

    if !migrations.is_empty() {
        session.permission("administrator").await?;
        session.permission("schema::write").await?;
    } 

    let salt = SaltString::from_b64(&state.cfg.password_salt).unwrap();

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .expect("Error occurred while encrypted password")
        .to_string();

    let migration_files = state.migration_service.get_migration_files().await?
        .iter()
        .filter(|value| !migrations.contains(value.as_str())).cloned().collect::<BTreeSet<String>>();

    for file in migration_files.iter() {
        let sql = state.migration_service.get_migration(file).await?;

        state
            .db
            .query(sql)
            .bind(("login", payload.login.trim().to_uppercase()))
            .bind(("password", password_hash.as_str()))
            .await?;
        info!("Migration {} is done!", file);

        migrations.insert(file.clone());

        state.system_service.set_migrations(migrations.clone()).await?;
    }

    Ok(ApiResponse::Ok)
}

pub async fn get_migrations_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<StringListModel> {
    session.permission("administrator").await?;
    session.permission("schema::write").await?;
    
    let migrations = StringListModel {
        list: state.system_service.get_migrations().await?.iter().cloned().collect::<Vec<String>>() 
    };
    
    migrations.ok_model()
}