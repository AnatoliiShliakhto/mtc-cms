use std::collections::BTreeSet;

use axum::async_trait;
use tokio::fs;

use crate::error::Result;

crate::impl_service!(MigrationService);

#[async_trait]
pub trait MigrationTrait {
    async fn get_migration_files(&self) -> Result<BTreeSet<String>>;
    async fn get_migration(&self, file_name: &str) -> Result<String>;
}

#[async_trait]
impl MigrationTrait for MigrationService {
    async fn get_migration_files(&self) -> Result<BTreeSet<String>> {
        let mut files = BTreeSet::<String>::new();
        if let Ok(mut folder) = fs::read_dir(&self.cfg.migration_path).await {
            while let Ok(Some(child)) = folder.next_entry().await {
                if let Ok(meta) = child.metadata().await {
                    let file_name = child.file_name().into_string().unwrap_or_default();
                    if meta.is_file() && file_name.contains(".sql") {
                        files.insert(file_name);
                    }
                }
            }
        }

        Ok(files)
    }

    async fn get_migration(&self, file_name: &str) -> Result<String> {
        Ok(fs::read_to_string([&self.cfg.migration_path, file_name].join("/")).await?)
    }
}
