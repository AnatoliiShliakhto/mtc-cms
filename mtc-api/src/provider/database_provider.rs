use once_cell::sync::Lazy;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;
use tracing::log::info;

use crate::provider::config_provider::CFG;

pub type Database = Surreal<Db>;

pub static DB: Lazy<Database> = Lazy::new(Surreal::init);

pub async fn db_init() {
    DB.connect::<RocksDb>(&CFG.db_url).await.unwrap();

    let version = DB.version().await.unwrap();
    info!("\x1b[38;5;6mSurreal DB version: \x1b[38;5;13m{version}\x1b[0m");

    DB.use_ns(&CFG.db_namespace).use_db(&CFG.db_name).await.unwrap();
    info!(
            "\x1b[38;5;6mSurrealDB \x1b[38;5;15mns: \x1b[38;5;13m{} \x1b[38;5;15mdb: \x1b[38;5;13m{}\x1b[0m",
            CFG.db_namespace.clone(),
            CFG.db_name.clone()
        );
}