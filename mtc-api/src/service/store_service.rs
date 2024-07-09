use axum::async_trait;
use axum::extract::multipart::Field;
use tokio::fs;

use mtc_model::store_model::{StoreModel, StoresModel};

use crate::error::Result;

crate::impl_service!(StoreService);

#[async_trait]
pub trait StoreTrait {
    fn get_dir_path(&self, dir: &str) -> String;
    fn get_file_path(&self, dir: &str, file: &str) -> String;
    async fn get_dir(&self, dir: &str) -> Result<StoresModel>;
    async fn is_dir_exists_or_create(&self, dir: &str) -> Result<bool>;
    async fn is_file_exists(&self, dir: &str, file: &str) -> Result<bool>;
    async fn remove_dir(&self, dir: &str) -> Result<bool>;
    async fn save_file(&self, dir: &str, data: Field<'_>) -> Result<()>;
    async fn delete_file(&self, dir: &str, file: &str) -> Result<()>;
}

#[async_trait]
impl StoreTrait for StoreService {
    fn get_dir_path(&self, dir: &str) -> String {
        [self.cfg.store_path.as_str(), dir].join("/")
    }

    fn get_file_path(&self, dir: &str, file: &str) -> String {
        [self.cfg.store_path.as_str(), dir, file].join("/")
    }

    async fn get_dir(&self, dir: &str) -> Result<StoresModel> {
        let mut stores = StoresModel::default();

        if let Ok(mut path) = fs::read_dir(self.get_dir_path(dir)).await {
            while let Ok(Some(child)) = path.next_entry().await {
                if let Ok(meta) = child.metadata().await {
                    if meta.is_file() {
                        stores.files.push(StoreModel {
                            name: child.file_name().into_string().unwrap_or_default(),
                            size: meta.len() as usize,
                        })
                    }
                }
            }
        }

        Ok(stores)
    }

    async fn is_dir_exists_or_create(&self, dir: &str) -> Result<bool> {
        let dir = self.get_dir_path(dir);

        match fs::metadata(&dir).await {
            Ok(path) => Ok(path.is_dir()),
            Err(_) => Ok(fs::create_dir_all(dir).await.is_ok()),
        }
    }

    async fn is_file_exists(&self, dir: &str, file: &str) -> Result<bool> {
        match fs::metadata(self.get_file_path(dir, file)).await {
            Ok(path) => Ok(path.is_file()),
            Err(_) => Ok(false),
        }
    }

    async fn remove_dir(&self, dir: &str) -> Result<bool> {
        Ok(fs::remove_dir_all(self.get_dir_path(dir)).await.is_ok())
    }

    async fn save_file(&self, dir: &str, data: Field<'_>) -> Result<()> {
        let file_path = self.get_file_path(dir, data.file_name().unwrap());

        fs::write(file_path, data.bytes().await?).await?;
        Ok(())
    }

    async fn delete_file(&self, dir: &str, file: &str) -> Result<()> {
        fs::remove_file(self.get_file_path(dir, file)).await?;
        
        Ok(())
    }
}
