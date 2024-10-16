use super::*;

pub async fn database_init(config: &Config) -> Result<Database> {
    let db = db_pre_init(&match cfg!(debug_assertions) {
        true => "127.0.0.1:8000".into(),
        false => config.db_path.clone()
    }).await?;
    
    let version = db.version().await?;
    info!("\x1b[38;5;6mSurrealDB \x1b[38;5;15mversion: \x1b[38;5;13m{version}\x1b[0m");

    db.use_ns(&*config.db_namespace).use_db(&*config.db_name).await?;
    info!(
            "\x1b[38;5;6mSurrealDB \x1b[38;5;15mns: \x1b[38;5;13m{} \x1b[38;5;15mdb: \x1b[38;5;13m{}\x1b[0m",
            config.db_namespace,
            config.db_name
        );

    Ok(db)
}

#[cfg(debug_assertions)]
async fn db_pre_init(database_path: &str) -> Result<Database> {
    let db =
        surrealdb::Surreal::new::<surrealdb::engine::remote::ws::Ws>(database_path).await?;
    db.signin(surrealdb::opt::auth::Root {
        username: "root",
        password: "root",
    }).await?;
    Ok(db)
}
#[cfg(not(debug_assertions))]
async fn db_pre_init(database_path: &str) -> Result<Database> {
    let db = Database::new::<surrealdb::engine::local::RocksDb>(database_path).await?;
    Ok(db)
}
