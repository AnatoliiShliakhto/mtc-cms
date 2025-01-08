use super::*;

pub trait SchemasRepository {
    fn find_schema_list(&self)
        -> impl Future<Output = Result<Vec<Entry>>> + Send;
    fn find_schema(&self, id: Cow<'static, str>)
        -> impl Future<Output = Result<Schema>> + Send;
    fn find_schema_by_slug(&self, slug: Cow<'static, str>)
        -> impl Future<Output = Result<Schema>> + Send;
    fn update_schema(&self, payload: Value, by: Cow<'static, str>)
        -> impl Future<Output = Result<()>> + Send;
    fn delete_schema(&self, id: Cow<'static, str>)
        -> impl Future<Output = Result<()>> + Send;
    fn find_pages_entries(
        &self,
        permissions: BTreeSet<Cow<'static, str>>,
    ) -> impl Future<Output = Result<Vec<Entry>>> + Send;
    fn find_schemas_records(&self)
        -> impl Future<Output = Result<Vec<Schema>>> + Send;
}

impl SchemasRepository for Repository {
    /// Retrieves a list of all schemas.
    ///
    /// # Description
    ///
    /// This method retrieves a list of all schemas from the database.
    ///
    /// # Return
    ///
    /// Returns a `Result` containing a vector of [`Entry`] objects representing the list of schemas.
    async fn find_schema_list(&self) -> Result<Vec<Entry>> {
        let sql = r#"
            SELECT record::id(id) as id, slug, title, created_at,
            { 'kind': kind, 'permission': permission } as variant
            FROM schemas WHERE kind > 1 ORDER BY created_at;
            "#;

        let schemas = self.database.query(sql)
            .await?
            .take::<Vec<Entry>>(0)?;

        Ok(schemas)
    }

    /// Retrieves a schema by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A `Cow<'static, str>` representing the ID of the schema to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the [`Schema`] if found.
    ///
    /// # Errors
    ///
    /// Returns an error if the schema is not found or if there is a database query failure.
    async fn find_schema(&self, id: Cow<'static, str>) -> Result<Schema> {
        let sql = r#"
            SELECT *, record::id(id) as id FROM ONLY type::record("schemas:" + $id)
            WHERE kind > 1;
        "#;

        self
            .database
            .query(sql)
            .bind(("id", id))
            .await?
            .take::<Option<Schema>>(0)?
            .ok_or(DatabaseError::EntryNotFound.into())
    }

    /// Retrieves a schema by its slug.
    ///
    /// # Arguments
    ///
    /// * `slug` - A [`Cow<str>`] representing the slug of the schema to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the retrieved [`Schema`] if found.
    ///
    /// # Errors
    ///
    /// Returns an error if the schema is not found or if there is a database query failure.
    async fn find_schema_by_slug(&self, slug: Cow<'static, str>) -> Result<Schema> {
        let sql = r#"
            SELECT *, record::id(id) as id FROM schemas
            WHERE slug = $slug LIMIT 1;
        "#;

        self
            .database
            .query(sql)
            .bind(("slug", slug))
            .await?
            .take::<Option<Schema>>(0)?
            .ok_or(DatabaseError::EntryNotFound.into())
    }

    /// Updates a schema.
    ///
    /// # Arguments
    ///
    /// * `payload` - A [`Value`] containing the schema data to update.
    /// * `by` - A `Cow<'static, str>` representing the user who is performing the update.
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if the schema is successfully updated.
    ///
    /// # Errors
    ///
    /// Returns an error if the schema is not found or if there is a database query failure.
    async fn update_schema(&self, payload: Value, by: Cow<'static, str>) -> Result<()> {
        let mut sql = vec!["BEGIN TRANSACTION;"];
        let id = payload.key_str("id").unwrap_or_default();
        let kind =
            SchemaKind::from_str(&payload.key_str("kind").unwrap_or_default())
            .unwrap_or_default();
        let slug = payload.key_str("slug").unwrap_or_default();
        let title = payload.key_str("title").unwrap_or_default();
        let permission = payload.key_str("permission").unwrap_or_default();
        let fields = payload.key_obj::<Vec<Field>>("fields")
            .unwrap_or_default();


        let content_slug: Cow<'static, str> = if id.is_empty() { "".into() } else {
            self
                .database
                .query(r#"SELECT VALUE slug FROM ONLY type::record("schemas:" + $id);"#)
                .bind(("id", id.clone()))
                .await?
                .take::<Option<Cow<'static, str>>>(0)?
                .unwrap_or_default()
        };

        if payload.contains_key("id") && !id.is_empty() {
            sql.push(r#"
            LET $rec_id = UPDATE type::record("schemas:" + $id) MERGE {
            "#)
        } else {
            sql.push(r#"
            LET $rec_id = CREATE schemas CONTENT {
                kind: $kind,
                created_by: $by,
            "#)
        }

        if payload.contains_key("slug") {
            sql.push(r#"
            slug: $slug,
            "#)
        }

        if payload.contains_key("title") {
            sql.push(r#"
            title: $title,
            "#)
        }

        if payload.contains_key("permission") {
            sql.push(r#"
            permission: $permission,
            "#)
        }

        if payload.contains_key("fields") {
            sql.push(r#"
            fields: $fields,
            "#)
        }

        sql.push(r#"
            updated_by: $by
        };
        "#);

        let sql_pages = format!(r#"
            DEFINE TABLE {0} SCHEMAFULL;
            DEFINE FIELD slug ON TABLE {0} TYPE string;
            DEFINE FIELD title ON TABLE {0} TYPE string;
            DEFINE FIELD data ON TABLE {0} FLEXIBLE TYPE option<object>;
            DEFINE FIELD published ON TABLE {0} TYPE bool DEFAULT false;
            DEFINE FIELD created_at ON TABLE {0} TYPE datetime DEFAULT time::now();
            DEFINE FIELD updated_at ON TABLE {0} TYPE datetime VALUE time::now();
            DEFINE FIELD created_by ON TABLE {0} TYPE string;
            DEFINE FIELD updated_by ON TABLE {0} TYPE string;
            DEFINE INDEX idx_{0}_slug ON TABLE {0} COLUMNS slug UNIQUE;
            "#, slug);

        if id.is_empty() {
            match kind {
                SchemaKind::Page => sql.push(r#"
                    LET $rec_id = CREATE page CONTENT {
	                    slug: $slug,
	                    title: $title,
	                    created_by: $by,
	                    updated_by: $by
                    };
                "#),
                SchemaKind::Pages => sql.push(&sql_pages),
                SchemaKind::Course => sql.push(r#"
                    LET $rec_id = CREATE course CONTENT {
	                    slug: $slug,
	                    title: $title,
	                    data: $data,
	                    created_by: $by,
	                    updated_by: $by
                    };
                "#),
                _ => {}
            }
        }

        if !id.is_empty() && slug.ne(&content_slug) && kind == SchemaKind::Page {
            sql.push(r#"
            UPDATE page SET slug = $slug WHERE slug = $content_slug;
            "#);
        }

        if !id.is_empty() && slug.ne(&content_slug) && kind == SchemaKind::Course {
            sql.push(r#"
            UPDATE course SET slug = $slug WHERE slug = $content_slug;
            "#);
        }

        sql.push("RETURN record::id($rec_id[0].id);\n");
        sql.push("COMMIT TRANSACTION;");

        let content_id = self
            .database
            .query(sql.concat())
            .bind(("id", id.clone()))
            .bind(("slug", slug))
            .bind(("content_slug", content_slug))
            .bind(("title", title))
            .bind(("kind", kind.clone()))
            .bind(("permission", permission))
            .bind(("fields", fields))
            .bind(("by", by))
            .bind(("data", json!({ "course": vec![CourseEntry::default()] })))
            .await?
            .take::<Option<Cow<'static, str>>>(0)?;

        if id.is_empty() &&
            (kind == SchemaKind::Course || kind == SchemaKind::Page)  {
            if let Some(id) = content_id {
                self.create_assets(&id).await?
            }
        }

        Ok(())
    }

    /// Deletes a schema and its associated records from the database.
    ///
    /// # Arguments
    ///
    /// * `id` - A `Cow<'static, str>` representing the ID of the schema to delete.
    ///
    /// # Description
    ///
    /// This method removes a schema record and its associated content from the database,
    /// including tables and assets related to the schema. It performs the deletion in a
    /// transaction to ensure atomicity. The method checks the schema kind and deletes
    /// associated records accordingly, such as pages or courses. If the schema has associated
    /// assets, they are also deleted.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the deletion is successful.
    ///
    /// # Errors
    ///
    /// Returns an error if the schema is not found, if there is a database query failure,
    /// or if the assets cannot be deleted.
    async fn delete_schema(&self, id: Cow<'static, str>) -> Result<()> {
        let schema = self.find_schema(id.clone()).await?;

        let mut sql = vec![r#"
            BEGIN TRANSACTION;
            DELETE type::record("schemas:" + $id) WHERE kind > 1;
        "#];

        let sql_remove_table = format!("REMOVE TABLE IF EXISTS {};", schema.slug);
        let mut sql_content_ids = "";

        match schema.kind {
            SchemaKind::Page => {
                sql.push(r#"
                    DELETE FROM page WHERE slug = $slug;
                "#);
                sql_content_ids = r#"SELECT VALUE record::id(id) FROM page WHERE slug=$slug;"#;
                self.delete_assets(&schema.id).await?
            },
            SchemaKind::Pages => {
                sql.push(&sql_remove_table);
                sql_content_ids = r#"SELECT VALUE record::id(id) FROM type::table($slug);"#;
            },
            SchemaKind::Course => {
                sql.push(r#"
                    DELETE FROM course WHERE slug = $slug;
                "#);
                sql_content_ids = r#"SELECT VALUE record::id(id) FROM course WHERE slug=$slug;"#;
                self.delete_assets(&schema.id).await?
            },
            _ => {}
        }

        sql.push(r#"COMMIT TRANSACTION;"#);

        if !sql_content_ids.is_empty() {
            let ids = self
                .database
                .query(sql_content_ids)
                .bind(("slug", schema.slug.clone()))
                .await?
                .take::<Vec<Cow<'static, str>>>(0)
                .unwrap_or_default();

            for id in ids {
                self.delete_assets(&id).await?
            }
        }

        self
            .database
            .query(sql.concat())
            .bind(("id", id))
            .bind(("slug", schema.slug))
            .await?;

        Ok(())
    }

    /// Retrieves a list of all page entries.
    ///
    /// # Description
    ///
    /// This method retrieves a list of all page entries from the database.
    ///
    /// # Arguments
    ///
    /// * `permissions` - A set of permissions to filter the schemas by.
    ///
    /// # Return
    ///
    /// Returns a `Result` containing a vector of [`Entry`] objects representing the list of page entries.
    async fn find_pages_entries(
        &self,
        permissions: BTreeSet<Cow<'static, str>>
    ) -> Result<Vec<Entry>> {
        let sql = r#"
            SELECT record::id(id) as id, slug, title
            FROM schemas WHERE kind = 3 AND permission in $permissions
            ORDER BY title;
            "#;

        let schemas = self.database.query(sql)
            .bind(("permissions", permissions))
            .await?
            .take::<Vec<Entry>>(0)?;

        Ok(schemas)
    }

    /// Retrieves a list of schema entries.
    ///
    /// # Description
    ///
    /// This method queries the database to retrieve a list of schema entries.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of [`Schema`] objects.
    ///
    /// # Errors
    ///
    /// Returns an error if there is a database query failure.
    async fn find_schemas_records(&self) -> Result<Vec<Schema>> {
        let sql = r#"
            SELECT *, record::id(id) as id FROM schemas WHERE kind > 1;
        "#;

        let schemas = self.database.query(sql)
            .await?
            .take::<Vec<Schema>>(0)?;

        Ok(schemas)
    }
}