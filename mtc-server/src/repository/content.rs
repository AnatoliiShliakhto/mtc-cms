use super::*;

#[async_trait]
pub trait ContentRepository {
    async fn find_content_list(&self, table: Cow<'static, str>, full: bool) -> Result<Vec<Entry>>;
    async fn find_content(
        &self,
        table: Cow<'static, str>,
        slug: Cow<'static, str>
    ) -> Result<Content>;
    async fn update_content(
        &self,
        table: Cow<'static, str>,
        current_slug: Cow<'static, str>,
        payload: Value,
        by: Cow<'static, str>,
    ) -> Result<()>;
    async fn delete_content(&self, table:Cow<'static, str>, slug: Cow<'static, str>) -> Result<()>;
    async fn get_course_files(
        &self,
        course_slug: Cow<'static, str>,
    ) -> Result<Vec<Cow<'static, str>>>;
    async fn drop_course_files(&self) -> Result<()>;
    async fn get_course_links(&self, course_slug: Cow<'static, str>) -> Result<Vec<FileEntry>>;
}

#[async_trait]
impl ContentRepository for Repository {
    async fn find_content_list(&self, table: Cow<'static, str>, full: bool) -> Result<Vec<Entry>> {
        let mut sql = vec![r#"
            SELECT record::id(id) as id, slug, title, published as variant, created_at
            FROM type::table($table)
        "#];

        if !full {
            sql.push(r#"
                WHERE published = true
            "#);
        }

        if table.eq("news") {
            sql.push(r#"
                ORDER BY created_at DESC;
            "#)
        } else {
            sql.push(r#"
                ORDER BY title ASC;
            "#)
        }

        let content_list = self.database.query(sql.concat())
            .bind(("table", table))
            .await?
            .take::<Vec<Entry>>(0)?;

        Ok(content_list)
    }

    async fn find_content(
        &self,
        table: Cow<'static, str>,
        slug: Cow<'static, str>
    ) -> Result<Content> {
        let sql = r#"
            SELECT *, record::id(id) as id FROM type::table($table) WHERE slug = $slug LIMIT 1;
        "#;

        self
            .database
            .query(sql)
            .bind(("table", table))
            .bind(("slug", slug))
            .await?
            .take::<Option<Content>>(0)?
        .ok_or(DatabaseError::EntryNotFound.into())
    }

    async fn update_content(
        &self,
        table: Cow<'static, str>,
        current_slug: Cow<'static, str>,
        payload: Value,
        by: Cow<'static, str>
    ) -> Result<()> {
        let mut sql = vec!["BEGIN TRANSACTION;"];
        let id =
            payload.key_str("id").unwrap_or_default();
        let slug =
            payload.key_str("slug").unwrap_or_default();
        let title =
            payload.key_str("title").unwrap_or_default();
        let published =
            payload.key_bool("published").unwrap_or_default();
        let data =
            payload.key_obj::<Value>("data").unwrap_or_default();

        if payload.contains_key("id") && !id.is_empty() {
            sql.push(r#"UPDATE type::record($table + ":" + $id) MERGE {"#)
        } else {
            sql.push(r#"
                CREATE type::table($table) CONTENT {
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

        if payload.contains_key("published") {
            sql.push(r#"
            published: $published,
            "#)
        }

        if payload.contains_key("data") {
            sql.push(r#"
            data: $data,
            "#)
        }

        sql.push(r#"
            updated_by: $by
        };
        "#);

        if !id.is_empty() & payload.contains_key("slug") & table.eq("page")
            & current_slug.ne(&slug) {
            sql.push(r#"
            UPDATE schemas SET slug = $slug WHERE slug = $current_slug;
            "#)
        }

        if !id.is_empty() & payload.contains_key("slug") & table.eq("course")
            & current_slug.ne(&slug) {
            sql.push(r#"
            UPDATE schemas SET slug = $slug WHERE slug = $current_slug;
            "#)
        }

        sql.push("COMMIT TRANSACTION;");

        self
            .database
            .query(sql.concat())
            .bind(("table", table))
            .bind(("id", id))
            .bind(("slug", slug))
            .bind(("current_slug", current_slug))
            .bind(("title", title))
            .bind(("published", published))
            .bind(("data", data))
            .bind(("by", by))
            .await?;

        Ok(())
    }

    async fn delete_content(&self, table:Cow<'static, str>, slug: Cow<'static, str>) -> Result<()> {
        let sql = r#"
            DELETE FROM type::table($table) WHERE slug = $slug;
        "#;

        self
            .database
            .query(sql)
            .bind(("table", table))
            .bind(("slug", slug))
            .await?;

        Ok(())
    }

    async fn get_course_files(
        &self,
        course_slug: Cow<'static, str>,
    ) -> Result<Vec<Cow<'static, str>>> {
        let sql = r#"
        BEGIN TRANSACTION;
        LET $courses = (SELECT array::group(array::group(data.course.links.{ url })) AS links
        FROM course WHERE slug = $slug GROUP ALL)[0].links;
        RETURN SELECT VALUE url FROM $courses WHERE type::is::string(url);
        COMMIT TRANSACTION;
        "#;

        let files = self.database.query(sql)
            .bind(("slug", course_slug))
            .await?
            .take::<Vec<Cow<'static, str>>>(0)?;

        Ok(files)
    }

    async fn drop_course_files(&self) -> Result<()> {
        let sql = r#"DELETE FROM course_files;"#;

        self.database.query(sql).await?;

        Ok(())
    }

    async fn get_course_links(&self, course_slug: Cow<'static, str>) -> Result<Vec<FileEntry>> {
        let sql = r#"
        SELECT name as path, size FROM course_files WHERE course = $slug;
        "#;

        let files = self.database.query(sql)
            .bind(("slug", course_slug))
            .await?
            .take::<Vec<FileEntry>>(0)?;

        Ok(files)
    }
}