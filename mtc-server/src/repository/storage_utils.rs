use super::*;

#[async_trait]
pub trait StorageTrait {
    fn get_dir_path(&self, dir: &str) -> String;
    fn get_file_path(&self, dir: &str, file: &str) -> String;
    fn get_private_dir_path(&self, dir: &str) -> String;
    fn get_private_file_path(&self, dir: &str, file: &str) -> String;
    async fn is_dir_exists_or_create(&self, path: &str) -> Result<bool>;
    async fn is_file_exists(&self, path: &str) -> Result<bool>;
    async fn remove_dir(&self, path: &str) -> Result<bool>;
    async fn delete_file(&self, path: &str) -> Result<()>;
    async fn create_assets(&self, id: &str) -> Result<()>;
    async fn delete_assets(&self, id: &str) -> Result<()>;
}

#[async_trait]
impl StorageTrait for Repository {
    fn get_dir_path(&self, dir: &str) -> String {
        [&self.config.storage_path, dir].join("/")
    }

    fn get_file_path(&self, dir: &str, file: &str) -> String {
        [&self.config.storage_path, dir, file].join("/")
    }

    fn get_private_dir_path(&self, dir: &str) -> String {
        [&self.config.private_storage_path, dir].join("/")
    }

    fn get_private_file_path(&self, dir: &str, file: &str) -> String {
        [&self.config.private_storage_path, dir, file].join("/")
    }

    async fn is_dir_exists_or_create(&self, path: &str) -> Result<bool> {
        match fs::metadata(&path).await {
            Ok(value) => Ok(value.is_dir()),
            Err(_) => Ok(fs::create_dir_all(path).await.is_ok()),
        }
    }

    async fn is_file_exists(&self, path: &str) -> Result<bool> {
        match fs::metadata(path).await {
            Ok(value) => Ok(value.is_file()),
            Err(_) => Ok(false),
        }
    }

    async fn remove_dir(&self, path: &str) -> Result<bool> {
        Ok(fs::remove_dir_all(path).await.is_ok())
    }

    async fn delete_file(&self, path: &str) -> Result<()> {
        fs::remove_file(path).await?;

        Ok(())
    }

    async fn create_assets(&self, id: &str) -> Result<()> {
        self.is_dir_exists_or_create(&self.get_dir_path(id)).await?;
        self.is_dir_exists_or_create(&self.get_private_dir_path(id))
            .await?;
        Ok(())
    }

    async fn delete_assets(&self, id: &str) -> Result<()> {
        self.remove_dir(&self.get_dir_path(id)).await?;
        self.remove_dir(&self.get_private_dir_path(id)).await?;
        Ok(())
    }
}
