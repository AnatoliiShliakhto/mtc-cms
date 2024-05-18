#[cfg(not(debug_assertions))]
use surrealdb::engine::local::{Db, SpeeDb};
#[cfg(debug_assertions)]
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tracing::log::info;

use crate::error::Result;
use crate::provider::config_provider::Config;

#[cfg(not(debug_assertions))]
pub type Database = Surreal<Db>;
#[cfg(debug_assertions)]
pub type Database = Surreal<Client>;

pub struct DatabaseProvider;

impl DatabaseProvider {
    pub async fn init(config: &Config) -> Result<Database> {
        let mut database_path = config.db_path.clone();
        if cfg!(debug_assertions) {
            database_path = "127.0.0.1:8000".to_string();
        }
        let db = db_pre_init(&database_path).await?;

        let version = db.version().await?;
        info!("\x1b[38;5;6mSurrealDB \x1b[38;5;15mversion: \x1b[38;5;13m{version}\x1b[0m");

        db.use_ns(&config.db_namespace).use_db(&config.db_name).await?;
        info!(
            "\x1b[38;5;6mSurrealDB \x1b[38;5;15mns: \x1b[38;5;13m{} \x1b[38;5;15mdb: \x1b[38;5;13m{}\x1b[0m",
            config.db_namespace.clone(),
            config.db_name.clone()
        );

        Ok(db)
    }
}

#[cfg(debug_assertions)]
async fn db_pre_init(database_path: &str) -> Result<Database> {
    let db = Database::new::<Ws>(database_path).await?;
    db.signin(Root {
        username: "root",
        password: "root",
    }).await?;
    Ok(db)
}

#[cfg(not(debug_assertions))]
async fn db_pre_init(database_path: &str) {
    let db = Database::new(Db).await?;
    db.connect::<SpeeDb>(database_path).await?;
    Ok(db)
}
