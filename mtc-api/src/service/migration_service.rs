use std::collections::BTreeSet;

use axum::async_trait;
use tokio::fs;
use crate::error::Result;

crate::impl_service!(MigrationService);

#[async_trait]
pub trait MigrationTrait {
    async fn is_new(&self) -> Result<bool>;
    async fn get_migration(&self) -> Result<Option<String>>;
}

#[async_trait]
impl MigrationTrait for MigrationService {
    async fn is_new(&self) -> Result<bool> {
        Ok(fs::try_exists([&self.cfg.migration_path, "init.sql"].join("/")).await?)
    }

    async fn get_migration(&self) -> Result<Option<String>> {
        let mut files = BTreeSet::<String>::new();
        if let Ok(mut folder) = fs::read_dir(&self.cfg.migration_path).await {
            while let Ok(Some(child)) = folder.next_entry().await {
                if let Ok(meta) = child.metadata().await {
                    if meta.is_file() {
                        files.insert(child.file_name().into_string().unwrap_or_default());
                    }
                }
            }
        }
        if files.is_empty() {
            return Ok(None);
        }
        
        let file_name = [&self.cfg.migration_path, files.first().unwrap().as_str()].join("/");
        let sql = fs::read_to_string(&file_name).await?;

        #[cfg(not(debug_assertions))]
        fs::remove_file(file_name).await?;
        
        Ok(Some(sql))
    }
}
