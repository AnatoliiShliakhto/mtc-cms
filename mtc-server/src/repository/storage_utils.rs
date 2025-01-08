use super::*;

pub trait StorageTrait {
    fn get_public_dir_path(&self, dir: &str) -> Cow<'static, str>;
    fn get_public_asset_path(&self, path: &str, file: &str) -> Cow<'static, str>;
    fn get_private_dir_path(&self, dir: &str) -> Cow<'static, str>;
    fn get_private_asset_path(&self, path: &str, file: &str) -> Cow<'static, str>;
    fn is_dir_exists_or_create(&self, path: &str)
        -> impl Future<Output = Result<bool>> + Send;
    fn is_file_exists(&self, path: &str)
        -> impl Future<Output = Result<bool>> + Send;
    fn remove_dir(&self, path: &str)
        -> impl Future<Output = Result<bool>> + Send;
    fn delete_file(&self, path: &str)
        -> impl Future<Output = Result<()>> + Send;
    fn create_assets(&self, id: &str)
        -> impl Future<Output = Result<()>> + Send;
    fn delete_assets(&self, id: &str)
        -> impl Future<Output = Result<()>> + Send;
    fn find_assets(&self, path: &str)
        -> impl Future<Output = Result<Vec<FileAsset>>> + Send;
    fn upload_asset(
        &self,
        path: &str,
        data: axum::extract::multipart::Field<'_>
    ) -> impl Future<Output = Result<()>> + Send;
    fn get_migration_files(&self)
        -> impl Future<Output = Result<BTreeSet<Cow<'static, str>>>> + Send;
    fn get_migration_file(&self, file_name: &str)
        -> impl Future<Output = Result<Cow<'static, str>>> + Send;
    fn update_course_files(
        &self,
        slug: Cow<'static, str>,
        files: Vec<Cow<'static, str>>
    ) -> impl Future<Output = Result<()>> + Send;
}

impl StorageTrait for Repository {
    /// Returns the path to the public directory specified by `dir` in the root of the storage path.
    fn get_public_dir_path(&self, dir: &str) -> Cow<'static, str> {
        [&self.config.storage_path, dir].join("/").into()
    }

    /// Returns the path to the public file specified by `path` and `file` in the root of the storage path.
    ///
    /// # Arguments
    ///
    /// * `path`: The relative path to the file.
    /// * `file`: The name of the file.
    ///
    /// # Returns
    ///
    /// The path to the file in the root of the storage path.
    fn get_public_asset_path(&self, path: &str, file: &str) -> Cow<'static, str> {
        [&self.config.storage_path, path, file].join("/").into()
    }

    /// Returns the path to the private directory specified by `dir` in the root of the private storage path.
    ///
    /// # Arguments
    ///
    /// * `dir`: The relative directory path.
    ///
    /// # Returns
    ///
    /// The path to the directory in the root of the private storage path.
    fn get_private_dir_path(&self, dir: &str) -> Cow<'static, str> {
        [&self.config.private_storage_path, dir].join("/").into()
    }

    /// Returns the path to the private file specified by `path` and `file` in the root of the private storage path.
    ///
    /// # Arguments
    ///
    /// * `path`: The relative path to the file.
    /// * `file`: The name of the file.
    ///
    /// # Returns
    ///
    /// The path to the file in the root of the private storage path.
    fn get_private_asset_path(&self, path: &str, file: &str) -> Cow<'static, str> {
        [&self.config.private_storage_path, path, file].join("/").into()
    }

    /// Checks if the directory at the given path exists, and if it does not, attempts to create it.
    ///
    /// # Arguments
    ///
    /// * `path`: The path to the directory.
    ///
    /// # Returns
    ///
    /// `true` if the directory exists, `false` otherwise.
    async fn is_dir_exists_or_create(&self, path: &str) -> Result<bool> {
        match fs::metadata(&path).await {
            Ok(value) => Ok(value.is_dir()),
            Err(_) => Ok(fs::create_dir_all(path).await.is_ok()),
        }
    }

    /// Checks if the file at the given path exists.
    ///
    /// # Arguments
    ///
    /// * `path`: The path to the file.
    ///
    /// # Returns
    ///
    /// `true` if the file exists, `false` otherwise.
    async fn is_file_exists(&self, path: &str) -> Result<bool> {
        match fs::metadata(path).await {
            Ok(value) => Ok(value.is_file()),
            Err(_) => Ok(false),
        }
    }

    /// Removes the directory at the specified path and all of its contents.
    ///
    /// # Arguments
    ///
    /// * `path`: The path to the directory to be removed.
    ///
    /// # Returns
    ///
    /// `true` if the directory and its contents were successfully removed, `false` otherwise.
    async fn remove_dir(&self, path: &str) -> Result<bool> {
        Ok(fs::remove_dir_all(path).await.is_ok())
    }

    /// Deletes the file at the given path.
    ///
    /// # Arguments
    ///
    /// * `path`: The path to the file to be deleted.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the file was successfully deleted, `Err` otherwise.
    async fn delete_file(&self, path: &str) -> Result<()> {
        fs::remove_file(path).await?;

        Ok(())
    }

    /// Creates the public and private directories for the given `id` if they do not already exist.
    ///
    /// This method is used to create the public and private directories for a course or a user when they are created.
    ///
    /// # Arguments
    ///
    /// * `id`: The ID of the course or user for which the directories should be created.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the directories were successfully created, `Err` otherwise.
    async fn create_assets(&self, id: &str) -> Result<()> {
        self.is_dir_exists_or_create(&self.get_public_dir_path(id)).await?;
        self.is_dir_exists_or_create(&self.get_private_dir_path(id))
            .await?;
        Ok(())
    }

    /// Deletes the public and private directories for the given `id` if they exist.
    ///
    /// # Arguments
    ///
    /// * `id`: The ID of the course or user for which the directories should be deleted.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the directories were successfully deleted, `Err` otherwise.
    async fn delete_assets(&self, id: &str) -> Result<()> {
        self.remove_dir(&self.get_public_dir_path(id)).await?;
        self.remove_dir(&self.get_private_dir_path(id)).await?;
        Ok(())
    }

    /// Finds and returns a list of file assets in the specified directory path.
    ///
    /// This function asynchronously reads the directory at the given path and collects
    /// all files into a vector of [`FileAsset`] objects, which include the file name and size.
    ///
    /// # Arguments
    ///
    /// * `path`: The path to the directory where assets should be located.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of [`FileAsset`] objects representing the files found in the directory,
    /// or an error if the directory cannot be read.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be accessed or read.
    async fn find_assets(&self, path: &str) -> Result<Vec<FileAsset>> {
        let mut storage_list = vec![];

        if let Ok(mut folder) = fs::read_dir(path).await {
            while let Ok(Some(child)) = folder.next_entry().await {
                if let Ok(meta) = child.metadata().await {
                    if meta.is_file() {
                        storage_list.push(FileAsset {
                            name: child.file_name().into_string().unwrap_or_default().into(),
                            size: meta.len() as usize,
                        })
                    }
                }
            }
        }

        Ok(storage_list)
    }

    /// Uploads the given file to the specified directory path.
    ///
    /// This function creates a file at the given path with the given file name
    /// and writes the contents of the given file to it.
    ///
    /// # Arguments
    ///
    /// * `path`: The path to the directory where the file should be uploaded.
    /// * `data`: The file to be uploaded, which must include the file name.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the file was uploaded successfully.
    async fn upload_asset(
        &self,
        path: &str,
        data: axum::extract::multipart::Field<'_>,
    ) -> Result<()> {
        let asset_path = [path, data.file_name().unwrap()].join("/");

        fs::write(&asset_path, data.bytes().await?).await?;

        Ok(())
    }

    /// Finds and returns a list of file names for SurQL migration files in the configured `migration_path`.
    ///
    /// # Returns
    ///
    /// A `Result` containing a set of file names for SurQL migration files, or an error if the configured
    /// `migration_path` cannot be accessed or read.
    ///
    /// # Errors
    ///
    /// Returns an error if the configured `migration_path` cannot be accessed or read.
    async fn get_migration_files(&self) -> Result<BTreeSet<Cow<'static, str>>> {
        let mut files = BTreeSet::<Cow<'static, str>>::new();
        if let Ok(mut folder) = fs::read_dir(&self.config.migration_path.to_string()).await {
            while let Ok(Some(child)) = folder.next_entry().await {
                if let Ok(meta) = child.metadata().await {
                    let file_name = child.file_name().into_string().unwrap_or_default();
                    if meta.is_file() && file_name.contains(".surql") {
                        files.insert(file_name.into());
                    }
                }
            }
        }

        Ok(files)
    }

    /// Finds and returns the contents of a SurQL migration file specified by the `file_name` parameter.
    ///
    /// The file is read from the configured `migration_path`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the contents of the file, or an error if the file cannot be accessed or read.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be accessed or read.
    async fn get_migration_file(&self, file_name: &str) -> Result<Cow<'static, str>> {
        Ok(fs::read_to_string([&self.config.migration_path, file_name].join("/")).await?.into())
    }

    /// Updates the `course_files` table with the provided list of files for a given course slug.
    ///
    /// This function begins a transaction, iterates over the list of files, and inserts each
    /// valid file entry into the `course_files` table with its associated course slug, name, and size.
    /// After processing all files, the transaction is committed.
    ///
    /// # Arguments
    ///
    /// * `slug`: The slug identifier for the course.
    /// * `files`: A vector containing file paths to be associated with the course.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation is successful, or an error if the transaction fails.
    ///
    /// # Errors
    ///
    /// Returns an error if there is a failure in querying the database.
    async fn update_course_files(
        &self,
        slug: Cow<'static, str>,
        files: Vec<Cow<'static, str>>
    ) -> Result<()> {
        let mut sql = vec![
            r#"
                BEGIN TRANSACTION;
            "#.to_string()
        ];

        for file in files {
            let Ok(meta) = fs::metadata(format!(
                "{}{}",
                self.config.data_path,
                file,
            )).await else { continue };
            if meta.is_file() {
                sql.push(format!(r#"
                INSERT INTO
	            course_files (course, name, size)
	            VALUES ('{0}', '{1}', {2});
                "#, slug, file, meta.len())
                );
            }
        }

        sql.push(r#"COMMIT TRANSACTION;"#.to_string());

        self
            .database
            .query(sql.concat())
            .await?;

        Ok(())
    }
}
