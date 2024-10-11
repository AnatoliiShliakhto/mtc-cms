use super::*;

#[async_trait]
pub trait SystemTrait {
    async fn find_migrations(&self) -> Result<BTreeSet<Cow<'static, str>>>;
    async fn update_migrations(&self, migrations: BTreeSet<Cow<'static, str>>) -> Result<()>;
    async fn migrate(
        &self,
        sql: Cow<'static, str>,
        user: Cow<'static, str>,
        password: Cow<'static, str>,
    ) -> Result<()>;

    async fn find_system_info(&self) -> Result<SystemInfo>;
    async fn insert_search_idx(
        &self,
        kind: SearchKind,
        title: Cow<'static, str>,
        url: Cow<'static, str>,
        permission: Cow<'static, str>,
    ) -> Result<()>;
    async fn rebuild_search_idx(&self) -> Result<()>;
    async fn search_idx_scan_page(
        &self,
        table: Cow<'static, str>,
        slug: Cow<'static, str>,
        schema: &Schema,
        info: &mut SystemInfo,
    );
    async fn search_idx_scan_links(
        &self,
        links: &Vec<LinkEntry>,
        permission: Cow<'static, str>,
        info: &mut SystemInfo,
    );
    async fn search_idx_scan_course(
        &self,
        slug: Cow<'static, str>,
        schema: &Schema,
        info: &mut SystemInfo,
    );
    async fn search_idx_drop(&self) -> Result<()>;
    async fn get_search_idx_count(&self) -> Result<i32>;
    async fn update_system_value(&self, key: Cow<'static, str>, value: Value) -> Result<()>;
}

#[async_trait]
impl SystemTrait for Repository {
    async fn find_migrations(&self) -> Result<BTreeSet<Cow<'static, str>>> {
        Ok(self
            .database
            .query(r#"SELECT VALUE c_value from mtc_system WHERE c_key = 'migrations';"#)
            .await?
            .take::<Option<BTreeSet<Cow<'static, str>>>>(0)?.unwrap_or_default())
    }

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

    async fn find_system_info(&self) -> Result<SystemInfo> {
        Ok(self
            .database
            .query(r#"SELECT VALUE c_value from mtc_system WHERE c_key = 'info';"#)
            .await?
            .take::<Option<SystemInfo>>(0)?.unwrap_or_default())
    }

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
            format!("/view/{}/{}", table, content.slug).into(),
            schema.permission.clone()
        ).await;

        info.pages += 1;

        let Some(fields) = schema.fields.clone() else { return };
        let Some(data) = content.data else { return };

        for field in fields {
            match field.kind {
                FieldKind::Html => {
                    if let Some(html) = data.get_str(&field.slug) {
                        info.media += html.matches(r#"class="media""#).count() as i32;
                    }
                }
                FieldKind::Links => {
                    if let Some(links) =
                        data.get_object::<Vec<LinkEntry>>(&field.slug) {
                        let _ = self.search_idx_scan_links(
                            &links,
                            schema.permission.clone(),
                            info
                        ).await;
                    }
                }
                _ => {}
            }
        }
    }

    async fn search_idx_scan_links(
        &self,
        links: &Vec<LinkEntry>,
        permission: Cow<'static, str>,
        info: &mut SystemInfo,
    ) {
        for link in links {
            if link.url.is_empty() { continue; }
            if link.url.starts_with("/view") | link.url.starts_with("/list") {
                let _ = self.insert_search_idx(
                    SearchKind::LocalLink,
                    link.title.clone(),
                    link.url.clone(),
                    permission.clone()
                ).await;
                info.pages += 1;
            } else {
                let extension = get_extension_from_filename(&link.url);
                if extension.is_none() | link.url.starts_with("http") {
                    let _ = self.insert_search_idx(
                        SearchKind::Link,
                        link.title.clone(),
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
                        link.title.clone(),
                        link.url.clone(),
                        permission.clone()
                    ).await;
                    info.files += 1;
                }
            }
        }
    }

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
            content.title.clone(),
            format!("/view/course/{}", content.slug).into(),
            schema.permission.clone()
        ).await;

        info.courses += 1;

        let Some(fields) = schema.fields.clone() else { return };
        let Some(data) = content.data else { return };

        for field in fields {
            match field.kind {
                FieldKind::Course => {
                    let Some(course) =
                        data.get_object::<Vec<CourseEntry>>(&field.slug) else { continue };

                    for item in course {
                        let _ = self.insert_search_idx(
                            SearchKind::Course,
                            item.title,
                            format!("/view/course/{}/{}", content.slug, item.id).into(),
                            schema.permission.clone()
                        ).await;
                        if let Some(links) = item.links {
                            if let Ok(links) =
                                serde_json::from_value::<Vec<LinkEntry>>(links) {
                                self.search_idx_scan_links(
                                    &links,
                                    schema.permission.clone(),
                                    info).await;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    async fn search_idx_drop(&self) -> Result<()> {
        self
            .database
            .query(r#"DELETE FROM search_index;"#)
            .await?;
        Ok(())
    }

    async fn get_search_idx_count(&self) -> Result<i32> {
        Ok(self
            .database
            .query(r#"count(SELECT 1 FROM search_index);"#)
            .await?
            .take::<Option<i32>>(0)?.unwrap_or_default())
    }

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
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    std::path::Path::new(filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
}