use super::*;

#[async_trait]
pub trait StorageTrait {
    fn get_public_dir_path(&self, dir: &str) -> Cow<'static, str>;
    fn get_public_asset_path(&self, path: &str, file: &str) -> Cow<'static, str>;
    fn get_private_dir_path(&self, dir: &str) -> Cow<'static, str>;
    fn get_private_asset_path(&self, path: &str, file: &str) -> Cow<'static, str>;
    async fn is_dir_exists_or_create(&self, path: &str) -> Result<bool>;
    async fn is_file_exists(&self, path: &str) -> Result<bool>;
    async fn remove_dir(&self, path: &str) -> Result<bool>;
    async fn delete_file(&self, path: &str) -> Result<()>;
    async fn create_assets(&self, id: &str) -> Result<()>;
    async fn delete_assets(&self, id: &str) -> Result<()>;
    async fn find_assets(&self, path: &str) -> Result<Vec<Asset>>;
    async fn upload_asset(
        &self,
        path: &str,
        data: axum::extract::multipart::Field<'_>
    ) -> Result<()>;
    async fn get_migration_files(&self) -> Result<BTreeSet<Cow<'static, str>>>;
    async fn get_migration_file(&self, file_name: &str) -> Result<Cow<'static, str>>;
}

#[async_trait]
impl StorageTrait for Repository {
    fn get_public_dir_path(&self, dir: &str) -> Cow<'static, str> {
        [&self.config.storage_path, dir].join("/").into()
    }

    fn get_public_asset_path(&self, path: &str, file: &str) -> Cow<'static, str> {
        [&self.config.storage_path, path, file].join("/").into()
    }

    fn get_private_dir_path(&self, dir: &str) -> Cow<'static, str> {
        [&self.config.private_storage_path, dir].join("/").into()
    }

    fn get_private_asset_path(&self, path: &str, file: &str) -> Cow<'static, str> {
        [&self.config.private_storage_path, path, file].join("/").into()
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
        self.is_dir_exists_or_create(&self.get_public_dir_path(id)).await?;
        self.is_dir_exists_or_create(&self.get_private_dir_path(id))
            .await?;
        Ok(())
    }

    async fn delete_assets(&self, id: &str) -> Result<()> {
        self.remove_dir(&self.get_public_dir_path(id)).await?;
        self.remove_dir(&self.get_private_dir_path(id)).await?;
        Ok(())
    }

    async fn find_assets(&self, path: &str) -> Result<Vec<Asset>> {
        let mut storage_list = vec![];

        if let Ok(mut folder) = fs::read_dir(path).await {
            while let Ok(Some(child)) = folder.next_entry().await {
                if let Ok(meta) = child.metadata().await {
                    if meta.is_file() {
                        storage_list.push(Asset {
                            name: child.file_name().into_string().unwrap_or_default().into(),
                            size: meta.len() as usize,
                        })
                    }
                }
            }
        }

        Ok(storage_list)
    }

    async fn upload_asset(
        &self,
        path: &str,
        data: axum::extract::multipart::Field<'_>,
    ) -> Result<()> {
        let asset_path = [path, data.file_name().unwrap()].join("/");

        fs::write(&asset_path, data.bytes().await?).await?;

        Ok(())
    }

    async fn get_migration_files(&self) -> Result<BTreeSet<Cow<'static, str>>> {
        let mut files = BTreeSet::<Cow<'static, str>>::new();
        if let Ok(mut folder) = fs::read_dir(&self.config.migration_path.to_string()).await {
            while let Ok(Some(child)) = folder.next_entry().await {
                if let Ok(meta) = child.metadata().await {
                    let file_name = child.file_name().into_string().unwrap_or_default();
                    if meta.is_file() && file_name.contains(".sql") {
                        files.insert(file_name.into());
                    }
                }
            }
        }

        Ok(files)
    }

    async fn get_migration_file(&self, file_name: &str) -> Result<Cow<'static, str>> {
        Ok(fs::read_to_string([&self.config.migration_path, file_name].join("/")).await?.into())
    }
}
