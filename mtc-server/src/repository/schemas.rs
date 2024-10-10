use super::*;

#[async_trait]
pub trait SchemasRepository {
    async fn find_schema_list(&self) -> Result<Vec<Entry>>;
    async fn find_schema(&self, id: Cow<'static, str>) -> Result<Schema>;
    async fn find_schema_by_slug(&self, slug: Cow<'static, str>) -> Result<Schema>;
    async fn update_schema(&self, payload: Value, by: Cow<'static, str>) -> Result<()>;
    async fn delete_schema(&self, id: Cow<'static, str>) -> Result<()>;
    async fn find_pages_entries(
        &self,
        permissions: BTreeSet<Cow<'static, str>>,
    ) -> Result<Vec<Entry>>;
    async fn find_schemas_records(&self) -> Result<Vec<Schema>>;
}

#[async_trait]
impl SchemasRepository for Repository {
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

    async fn update_schema(&self, payload: Value, by: Cow<'static, str>) -> Result<()> {
        let mut sql = vec!["BEGIN TRANSACTION;"];
        let id = payload.get_str("id").unwrap_or_default();
        let kind =
            SchemaKind::from_str(&payload.get_str("kind").unwrap_or_default())
            .unwrap_or_default();
        let slug = payload.get_str("slug").unwrap_or_default();
        let title = payload.get_str("title").unwrap_or_default();
        let permission = payload.get_str("permission").unwrap_or_default();
        let fields = payload.get_schema_fields().unwrap_or(vec![]);

        let content_slug = self
            .database
            .query(r#"SELECT VALUE slug FROM ONLY type::record("schemas:" + $id);"#)
            .bind(("id", id.clone()))
            .await?
            .take::<Option<Cow<'static, str>>>(0)?
            .unwrap_or_default();

        if payload.has_key("id") && !id.is_empty() {
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

        if payload.has_key("slug") {
            sql.push(r#"
            slug: $slug,
            "#)
        }

        if payload.has_key("title") {
            sql.push(r#"
            title: $title,
            "#)
        }

        if payload.has_key("permission") {
            sql.push(r#"
            permission: $permission,
            "#)
        }

        if payload.has_key("fields") {
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

        error!("{:#?}", sql.concat());
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