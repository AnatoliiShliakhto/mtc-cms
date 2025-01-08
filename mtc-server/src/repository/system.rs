use super::*;

pub trait SystemTrait {
    fn find_migrations(&self)
        -> impl Future<Output = Result<BTreeSet<Cow<'static, str>>>> + Send;
    fn update_migrations(&self, migrations: BTreeSet<Cow<'static, str>>)
        -> impl Future<Output = Result<()>> + Send;
    fn migrate(
        &self,
        sql: Cow<'static, str>,
        user: Cow<'static, str>,
        password: Cow<'static, str>,
    ) -> impl Future<Output = Result<()>> + Send;

    fn find_system_info(&self)
        -> impl Future<Output = Result<SystemInfo>> + Send;
    fn insert_search_idx(
        &self,
        kind: SearchKind,
        title: Cow<'static, str>,
        url: Cow<'static, str>,
        permission: Cow<'static, str>,
    ) -> impl Future<Output = Result<()>> + Send;
    fn rebuild_search_idx(&self)
        -> impl Future<Output = Result<()>> + Send;
    fn search_idx_scan_page(
        &self,
        table: Cow<'static, str>,
        slug: Cow<'static, str>,
        schema: &Schema,
        info: &mut SystemInfo,
    ) -> impl Future<Output = ()> + Send;
    fn search_idx_scan_links(
        &self,
        links: &Vec<LinkEntry>,
        permission: Cow<'static, str>,
        info: &mut SystemInfo,
        subtitle: Option<Cow<'static, str>>,
    ) -> impl Future<Output = ()> + Send;
    fn search_idx_scan_course(
        &self,
        slug: Cow<'static, str>,
        schema: &Schema,
        info: &mut SystemInfo,
    ) -> impl Future<Output = ()> + Send;
    fn search_idx_drop(&self)
        -> impl Future<Output = Result<()>> + Send;
    fn get_search_idx_count(&self)
        -> impl Future<Output = Result<i32>> + Send;
    fn get_system_value(&self, key: Cow<'static, str>)
        -> impl Future<Output = Result<Value>> + Send;
    fn update_system_value(&self, key: Cow<'static, str>, value: Value)
        -> impl Future<Output = Result<()>> + Send;
    fn sitemap_build(&self)
        -> impl Future<Output = Result<()>> + Send;
}

impl SystemTrait for Repository {
    /// Finds a set of all migrations that have been applied to the database.
    async fn find_migrations(&self) -> Result<BTreeSet<Cow<'static, str>>> {
        Ok(self
            .database
            .query(r#"SELECT VALUE c_value from mtc_system WHERE c_key = 'migrations';"#)
            .await?
            .take::<Option<BTreeSet<Cow<'static, str>>>>(0)?.unwrap_or_default())
    }

    /// Updates the set of migrations that have been applied to the database.
    async fn update_migrations(&self, migrations: BTreeSet<Cow<'static, str>>) -> Result<()> {
        self.database
            .query(
                r#"
                UPDATE mtc_system MERGE {
	                c_value: $value,
                } WHERE c_key = 'migrations';
                "#,
            )
            .bind(("value", migrations.into_iter().collect::<Vec<Cow<'static, str>>>()))
            .await?;

        Ok(())
    }

    /// Executes a migration SQL script on the database.
    ///
    /// This function runs the provided SQL script with the specified user credentials.
    ///
    /// # Arguments
    ///
    /// * `sql` - The SQL script to be executed as part of the migration.
    /// * `user` - The username to authenticate the migration process.
    /// * `password` - The password for the given user.
    async fn migrate(
        &self,
        sql: Cow<'static, str>,
        user: Cow<'static, str>,
        password: Cow<'static, str>
    ) -> Result<()> {
        self.database
            .query(&*sql)
            .bind(("login", user))
            .bind(("password", password))
            .await?;

        Ok(())
    }

    /// Finds the system information stored in the database.
    ///
    /// This function returns a [`SystemInfo`] object containing the system
    /// information, or an empty [`SystemInfo`] object if there is no system
    /// information stored in the database.
    ///
    /// # Errors
    ///
    /// Returns a `GenericError` if the query fails.
    async fn find_system_info(&self) -> Result<SystemInfo> {
        Ok(self
            .database
            .query(r#"SELECT VALUE c_value from mtc_system WHERE c_key = 'info';"#)
            .await?
            .take::<Option<SystemInfo>>(0)?.unwrap_or_default())
    }

    /// Inserts a search index entry into the database.
    ///
    /// This function creates a new search index entry in the database with the
    /// specified `kind`, `title`, `url`, and `permission`.
    ///
    /// # Arguments
    ///
    /// * `kind` - The type of search index entry to be inserted.
    /// * `title` - The title of the search index entry.
    /// * `url` - The URL of the search index entry.
    /// * `permission` - The permission required to view the search index entry.
    ///
    /// # Errors
    ///
    /// Returns a `GenericError` if the query fails.
    async fn insert_search_idx(
        &self,
        kind: SearchKind,
        title: Cow<'static, str>,
        url: Cow<'static, str>,
        permission: Cow<'static, str>
    ) -> Result<()> {
        let sql = r#"
            CREATE search_index CONTENT {
                kind: $kind,
                title: $title,
                url: $url,
                permission: $permission
            };
        "#;

        self
            .database
            .query(sql)
            .bind(("kind", kind))
            .bind(("title", title))
            .bind(("url", url))
            .bind(("permission", permission))
            .await?;

        Ok(())
    }

    /// Rebuilds the search index in the database.
    ///
    /// This function rebuilds the search index in the database by dropping the
    /// existing search index, then scanning all content and course records and
    /// inserting the necessary search index entries. It also updates the system
    /// information stored in the database.
    ///
    /// # Errors
    ///
    /// Returns a `GenericError` if any of the queries fail.
    async fn rebuild_search_idx(&self) -> Result<()> {
        let mut info = SystemInfo::default();

        self.search_idx_drop().await?;

        for schema in self.find_schemas_records().await? {
            match schema.kind {
                SchemaKind::Page => {
                    self.search_idx_scan_page(
                        "page".into(),
                        schema.slug.clone(),
                        &schema,
                        &mut info
                    ).await;
                }
                SchemaKind::Pages => {
                    if let Ok(pages) =
                        self.find_content_list(schema.slug.clone(), false).await {
                        for page in pages {
                            self.search_idx_scan_page(
                                schema.slug.clone(),
                                page.slug,
                                &schema,
                                &mut info
                            ).await;
                        }
                    }
                }
                SchemaKind::Course => {
                    self.search_idx_scan_course(
                        schema.slug.clone(),
                        &schema,
                        &mut info
                    ).await;
                }
                _ => {}
            }
        }

        info.active_users = self.get_users_count(true).await?;
        info.users = self.get_users_count(false).await?;
        info.indexes = self.get_search_idx_count().await?;

        self.update_system_value("info".into(), json!(info)).await?;

        Ok(())
    }

    /// Scans a page record and updates the search index and system information.
    ///
    /// This function takes a `table` name and a `slug` of a page, a
    /// [`Schema`] object, and a mutable reference to a [`SystemInfo`] object.
    ///
    /// It queries the database for the page record specified by the
    /// `table` and `slug`, then inserts a search index entry for the
    /// page. It also increments the `pages` field of the [`SystemInfo`]
    /// object.
    ///
    /// Additionally, if the page record has a field of type `Html`, it
    /// counts the number of media elements in the field and adds it to
    /// the `media` field of the [`SystemInfo`] object. If the page record
    /// has a field of type `Links`, it scans the links and updates the
    /// search index and system information accordingly.
    ///
    /// # Errors
    ///
    /// Returns a `GenericError` if the query fails.
    async fn search_idx_scan_page(
        &self,
        table: Cow<'static, str>,
        slug: Cow<'static, str>,
        schema: &Schema,
        info: &mut SystemInfo,
    ) {
        let sql = r#"
            SELECT *, record::id(id) as id FROM type::table($table)
            WHERE slug = $slug AND published = true;
        "#;

        let Ok(mut response) = self
            .database
            .query(sql)
            .bind(("table", table.clone()))
            .bind(("slug", slug))
            .await else { return };

        let Ok(Some(content)) = response.take::<Option<Content>>(0) else { return };

        let _ = self.insert_search_idx(
            SearchKind::LocalLink,
            content.title.clone(),
            format!("/content/{}/{}", table, content.slug).into(),
            schema.permission.clone()
        ).await;

        info.pages += 1;

        let Some(fields) = schema.fields.clone() else { return };
        let Some(data) = content.data else { return };

        for field in fields {
            match field.kind {
                FieldKind::Html => {
                    if let Some(html) = data.key_str(&field.slug) {
                        info.media += html.matches(r#"class="media""#).count() as i32;
                    }
                }
                FieldKind::Links => {
                    if let Some(links) =
                        data.key_obj::<Vec<LinkEntry>>(&field.slug) {
                        let _ = self.search_idx_scan_links(
                            &links,
                            schema.permission.clone(),
                            info,
                            None
                        ).await;
                    }
                }
                _ => {}
            }
        }
    }

    /// Scans a list of links and inserts them into the search index.
    ///
    /// If the link URL starts with "/content", it is treated as a local link
    /// and inserted into the search index as a `SearchKind::LocalLink`.
    ///
    /// If the link URL does not start with "/content", it is checked whether
    /// it has a file extension. If it does not have an extension, it is
    /// inserted into the search index as a `SearchKind::Link`. If it has an
    /// extension, it is inserted into the search index with the corresponding
    /// `SearchKind` value (e.g. `SearchKind::FilePdf` for PDF files).
    ///
    /// The `info` parameter is used to update the corresponding counter in
    /// the `SystemInfo` object. The `subtitle` parameter is used to construct
    /// a title for the link in the search index.
    ///
    /// # Errors
    ///
    /// Returns a `GenericError` if any of the queries fail.
    async fn search_idx_scan_links(
        &self,
        links: &Vec<LinkEntry>,
        permission: Cow<'static, str>,
        info: &mut SystemInfo,
        subtitle: Option<Cow<'static, str>>,
    ) {
        for link in links {
            if link.url.is_empty() { continue; }
            if link.url.starts_with("/content") {
                let _ = self.insert_search_idx(
                    SearchKind::LocalLink,
                    if let Some(sub) = subtitle.clone() {
                        format!("[{}] {}", sub, link.title).into()
                    } else {
                        link.title.clone()
                    },
                    link.url.clone(),
                    permission.clone()
                ).await;
                info.pages += 1;
            } else {
                let extension = get_extension_from_filename(&link.url);
                if extension.is_none() | link.url.starts_with("http") {
                    let _ = self.insert_search_idx(
                        SearchKind::Link,
                        if let Some(sub) = subtitle.clone() {
                            format!("[{}] {}", sub, link.title).into()
                        } else {
                            link.title.clone()
                        },
                        link.url.clone(),
                        permission.clone()
                    ).await;
                    info.links += 1;
                } else {
                    let _ = self.insert_search_idx(
                        match extension.unwrap_or_default() {
                            "xls" | "xlsx" | "xlsm" => SearchKind::FileExcel,
                            "doc" | "docx" | "docm" => SearchKind::FileWord,
                            "pptx" | "pptm" => SearchKind::FilePowerPoint,
                            "pdf" => SearchKind::FilePdf,
                            _ => SearchKind::File,
                        },
                        if let Some(sub) = subtitle.clone() {
                            format!("[{}] {}", sub, link.title).into()
                        } else {
                            link.title.clone()
                        },
                        link.url.clone(),
                        permission.clone()
                    ).await;
                    info.files += 1;
                }
            }
        }
    }

    /// Scans a course record and updates the search index and system information.
    ///
    /// This function takes a `slug` of a course, a
    /// [`Schema`] object, and a mutable reference to a
    /// [`SystemInfo`] object.
    ///
    /// It queries the database for the course record specified by the
    /// `slug`, then inserts a search index entry for the course. It also
    /// increments the `courses` field of the [`SystemInfo`]
    /// object.
    ///
    /// Additionally, if the course record has a field of type `Course`, it
    /// scans the course items and updates the search index and system
    /// information accordingly.
    ///
    /// # Errors
    ///
    /// Returns a `GenericError` if the query fails.
    async fn search_idx_scan_course(
        &self, slug: Cow<'static, str>,
        schema: &Schema,
        info: &mut SystemInfo,
    ) {
        let sql = r#"
            SELECT *, record::id(id) as id FROM course
            WHERE slug = $slug AND published = true;
        "#;

        let Ok(mut response) = self
            .database
            .query(sql)
            .bind(("slug", slug))
            .await else { return };

        let Ok(Some(content)) = response.take::<Option<Content>>(0) else { return };

        let _ = self.insert_search_idx(
            SearchKind::LocalLink,
            format!("[{}] {}", schema.title, content.title).into(),
            format!("/content/course/{}", content.slug).into(),
            schema.permission.clone()
        ).await;

        info.courses += 1;

        let Some(fields) = schema.fields.clone() else { return };
        let Some(data) = content.data else { return };

        for field in fields {
            match field.kind {
                FieldKind::Course => {
                    let Some(course) =
                        data.key_obj::<Vec<CourseEntry>>(&field.slug) else { continue };

                    for item in course {
                        let _ = self.insert_search_idx(
                            SearchKind::Course,
                            format!("[{}] {}", schema.title, item.title).into(),
                            format!("/content/course/{}/{}", content.slug, item.id).into(),
                            schema.permission.clone()
                        ).await;
                        if let Some(links) = item.links {
                            if let Ok(links) =
                                serde_json::from_value::<Vec<LinkEntry>>(links) {
                                self.search_idx_scan_links(
                                    &links,
                                    schema.permission.clone(),
                                    info,
                                    Some(schema.title.clone())
                                ).await;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Drops and recreates the full text search index.
    ///
    /// This method is useful for rebuilding the search index after inserting a large number of records.
    ///
    /// Returns a `GenericError` if the query fails.
    async fn search_idx_drop(&self) -> Result<()> {
        self
            .database
            .query(r#"
                BEGIN TRANSACTION;
                REMOVE INDEX IF EXISTS idx_search_title ON TABLE search_index;
                DELETE FROM search_index;
                DEFINE INDEX idx_search_title ON search_index
                FIELDS title SEARCH ANALYZER search_lowercase BM25;
                COMMIT TRANSACTION;
                "#)
            .await?;
        Ok(())
    }

    /// Returns the number of records in the search index.
    ///
    /// # Errors
    ///
    /// Returns a `GenericError` if the query fails.
    async fn get_search_idx_count(&self) -> Result<i32> {
        Ok(self
            .database
            .query(r#"count(SELECT 1 FROM search_index);"#)
            .await?
            .take::<Option<i32>>(0)?.unwrap_or_default())
    }

    /// Retrieves a system value from the database based on the provided key.
    ///
    /// This function queries the `mtc_system` table for a value associated with
    /// the specified key. If the key does not exist in the database, an empty
    /// [`Value`] is returned.
    ///
    /// # Arguments
    ///
    /// * `key` - The key for which the system value is to be retrieved.
    ///
    /// # Errors
    ///
    /// Returns a `GenericError` if the query execution fails.
    ///
    /// # Returns
    ///
    /// Returns the system value associated with the specified key, or an empty
    /// [`Value`] if the key does not exist.
    async fn get_system_value(&self, key: Cow<'static, str>) -> Result<Value> {
        Ok(self
            .database
            .query(r#"SELECT VALUE c_value from mtc_system WHERE c_key = $key;"#)
            .bind(("key", key))
            .await?
            .take::<Option<Value>>(0)?.unwrap_or_default())
    }

    /// Updates a system value in the database.
    ///
    /// This function queries the `mtc_system` table and updates the value associated with the
    /// specified key. If the key does not exist in the database, a new record is inserted.
    ///
    /// # Arguments
    ///
    /// * `key` - The key for which the system value is to be updated.
    /// * `value` - The value to be stored in the database for the specified key.
    ///
    /// # Errors
    ///
    /// Returns a `GenericError` if the query execution fails.
    ///
    /// # Returns
    ///
    /// Returns an empty `Result` on successful update.
    async fn update_system_value(&self, key: Cow<'static, str>, value: Value) -> Result<()> {
        self.database
            .query(
                r#"
                BEGIN TRANSACTION;
                DELETE FROM mtc_system WHERE c_key = $key;
                CREATE mtc_system CONTENT {
                    c_key: $key,
	                c_value: $value,
                };
                COMMIT TRANSACTION;
                "#,
            )
            .bind(("key", key))
            .bind(("value", value))
            .await?;

        Ok(())
    }

    /// Builds the sitemap for the system.
    ///
    /// This function generates a sitemap containing all visible pages, and
    /// writes it to a file named `sitemap.xml` in the `www_path` directory.
    ///
    /// # Errors
    ///
    /// Returns a `GenericError` if the query execution fails, or if there is an
    /// error writing the sitemap file.
    ///
    /// # Returns
    ///
    /// Returns an empty `Result` on successful sitemap build.
    async fn sitemap_build(&self) -> Result<()> {
        let url = format!(
            "https://{}",
            self.config.front_end_url
                .replace("/", "")
                .replace("https:", "").
                to_string()
        );
        let mut count = 2;

        let mut sitemap = vec![
            r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string(),
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#.to_string(),
            format!(r#"<url><loc>{0}</loc></url>"#, url),
            format!(r#"<url><loc>{0}/home</loc></url>"#, url),
        ];

        for schema in self.find_schemas_records().await? {
            if schema.permission.ne(PERMISSION_PUBLIC) { continue }

            match schema.kind {
                SchemaKind::Page => {
                    count += 1;
                    sitemap.push(
                        format!(r#"<url><loc>{}/content/{}/{}</loc></url>"#,
                                url, "page", schema.slug)
                    )
                }
                SchemaKind::Pages => {
                    if let Ok(pages) =
                        self.find_content_list(schema.slug.clone(), false).await {
                        count += 1;
                        sitemap.push(
                            format!(r#"<url><loc>{}/content/{}</loc></url>"#,
                                    url, schema.slug)
                        );
                        for page in pages {
                            count += 1;
                            sitemap.push(
                                format!(r#"<url><loc>{}/content/{}/{}</loc></url>"#,
                                        url, schema.slug, page.slug)
                            )
                        }
                    }
                }
                _ => {}
            }
        }
        sitemap.push(r#"</urlset>"#.to_string());
        self.update_system_value("sitemap".into(), Value::from(count)).await?;

        tokio::fs::write(
            format!("{}/sitemap.xml", self.config.www_path),
            sitemap.join("\n"),
        ).await?;

        Ok(())
    }
}

/// Gets the file extension from the given `filename`.
///
/// # Arguments
///
/// * `filename` - The file name from which the extension is to be extracted.
///
/// # Returns
///
/// Returns an `Option` containing the file extension if it exists,
/// otherwise an empty `Option` is returned.
fn get_extension_from_filename(filename: &str) -> Option<&str> {
    std::path::Path::new(filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
}