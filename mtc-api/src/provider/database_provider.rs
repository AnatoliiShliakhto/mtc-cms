use once_cell::sync::Lazy;
#[cfg(not(debug_assertions))]
use surrealdb::engine::local::{Db, SpeeDb};
#[cfg(debug_assertions)]
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tracing::log::info;

use crate::provider::config_provider::CFG;

pub static DB: Lazy<Database> = Lazy::new(Surreal::init);

// Standalone SurrealDB for DEBUG
#[cfg(debug_assertions)]
pub type Database = Surreal<Client>;

#[cfg(debug_assertions)]
async fn db_pre_init() {
    DB.connect::<Ws>("127.0.0.1:8000").await.unwrap();
    DB.signin(Root {
        username: "root",
        password: "root",
    })
        .await.unwrap();
}

// Embedded DB for RELEASE
#[cfg(not(debug_assertions))]
pub type Database = Surreal<Db>;

#[cfg(not(debug_assertions))]
async fn db_pre_init() {
    DB.connect::<SpeeDb>(&CFG.db_url).await.unwrap();
}

pub async fn db_init() {
    db_pre_init().await;

    let version = DB.version().await.unwrap();
    info!("\x1b[38;5;6mSurreal DB version: \x1b[38;5;13m{version}\x1b[0m");

    DB.use_ns(&CFG.db_namespace).use_db(&CFG.db_name).await.unwrap();
    info!(
            "\x1b[38;5;6mSurrealDB \x1b[38;5;15mns: \x1b[38;5;13m{} \x1b[38;5;15mdb: \x1b[38;5;13m{}\x1b[0m",
            CFG.db_namespace.clone(),
            CFG.db_name.clone()
        );
}