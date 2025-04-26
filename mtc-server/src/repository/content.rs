use super::*;

pub trait ContentRepository {
    async fn find_content_list(&self, table: impl ToString, full: bool) -> Result<Vec<Entry>>;
    async fn find_content(
        &self,
        table: impl ToString,
        slug: impl ToString,
    ) -> Result<Content>;
    async fn update_content(
        &self,
        table: impl ToString,
        current_slug: impl ToString,
        payload: Value,
        by: impl ToString,
    ) -> Result<()>;
    async fn delete_content(&self, table: impl ToString, slug: impl ToString) -> Result<()>;
    async fn get_course_files(
        &self,
        course_slug: impl ToString,
    ) -> Result<Vec<Cow<'static, str>>>;
    async fn drop_course_files(&self) -> Result<()>;
    async fn get_course_links(&self, course_slug: impl ToString) -> Result<Vec<FileEntry>>;
}

impl ContentRepository for Repository {
    async fn find_content_list(&self, table: impl ToString, full: bool) -> Result<Vec<Entry>> {
        let table = table.to_string();

        let mut sql = r#"
            SELECT id.id() as id, slug, title, published as variant, created_at
            FROM type::table($table)
        "#.to_string();

        if !full {
            sql.write_str(r#" WHERE published = true "#)?;
        }

        if table.eq("news") {
            sql.write_str(r#" ORDER BY created_at DESC; "#)?
        } else {
            sql.write_str(r#" ORDER BY title ASC; "#)?
        }

        self.database.query(sql)
            .bind(("table", table))
            .await?
            .take::<Vec<Entry>>(0)
            .map(Ok)?
    }

    async fn find_content(
        &self,
        table: impl ToString,
        slug: impl ToString,
    ) -> Result<Content> {
        let sql = r#"
            SELECT *, id.id() as id FROM type::table($table) WHERE slug = $slug LIMIT 1;
        "#;

        self
            .database
            .query(sql)
            .bind(("table", table.to_string()))
            .bind(("slug", slug.to_string()))
            .await?
            .take::<Option<Content>>(0)?
            .ok_or(DatabaseError::EntryNotFound.into())
    }

    async fn update_content(
        &self,
        table: impl ToString,
        current_slug: impl ToString,
        payload: Value,
        by: impl ToString,
    ) -> Result<()> {
        let table = table.to_string();
        let current_slug = current_slug.to_string();

        let mut sql = "BEGIN TRANSACTION;".to_string();
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
            sql.write_str(r#"
                UPDATE type::record($table + ":" + $id) MERGE {
            "#)?
        } else {
            sql.write_str(r#"
                CREATE type::table($table) CONTENT {
                created_by: $by,
            "#)?
        }

        if !slug.is_empty() {
            sql.write_str(r#"slug: $slug,"#)?
        }

        if !title.is_empty() {
            sql.write_str(r#"title: $title,"#)?
        }

        sql.write_str(r#"published: $published,"#)?;

        if !data.is_null() {
            sql.write_str(r#"data: $data,"#)?
        }

        sql.write_str(r#"updated_by: $by};"#)?;

        if !id.is_empty() && !slug.is_empty() && table.eq("page")
            & current_slug.ne(&slug) {
            sql.write_str(r#"
            UPDATE schemas SET slug = $slug WHERE slug = $current_slug;
            "#)?
        }

        if !id.is_empty() && !slug.is_empty() && table.eq("course")
            & current_slug.ne(&slug) {
            sql.write_str(r#"
            UPDATE schemas SET slug = $slug WHERE slug = $current_slug;
            "#)?
        }

        sql.write_str("COMMIT TRANSACTION;")?;

        self
            .database
            .query(sql)
            .bind(("table", table))
            .bind(("id", id))
            .bind(("slug", slug))
            .bind(("current_slug", current_slug))
            .bind(("title", title))
            .bind(("published", published))
            .bind(("data", data))
            .bind(("by", by.to_string()))
            .await?
            .check()?;

        Ok(())
    }

    async fn delete_content(&self, table: impl ToString, slug: impl ToString) -> Result<()> {
        let sql = r#"
            DELETE FROM type::table($table) WHERE slug = $slug;
        "#;

        self
            .database
            .query(sql)
            .bind(("table", table.to_string()))
            .bind(("slug", slug.to_string()))
            .await?
            .check()?;

        Ok(())
    }

    async fn get_course_files(
        &self,
        course_slug: impl ToString,
    ) -> Result<Vec<Cow<'static, str>>> {
        let sql = r#"
        BEGIN TRANSACTION;
        LET $courses = (SELECT array::group(array::group(data.course.links.{ url })) AS links
        FROM course WHERE slug = $slug GROUP ALL)[0].links;
        RETURN SELECT VALUE url FROM $courses WHERE type::is::string(url);
        COMMIT TRANSACTION;
        "#;

        self.database.query(sql)
            .bind(("slug", course_slug.to_string()))
            .await?
            .take::<Vec<Cow<'static, str>>>(0)
            .map(Ok)?
    }

    async fn drop_course_files(&self) -> Result<()> {
        let sql = r#"DELETE FROM course_files;"#;

        self.database.query(sql).await?.check()?;

        Ok(())
    }

    async fn get_course_links(&self, course_slug: impl ToString) -> Result<Vec<FileEntry>> {
        let sql = r#"
        SELECT name as path, size FROM course_files WHERE course = $slug;
        "#;

        self.database.query(sql)
            .bind(("slug", course_slug.to_string()))
            .await?
            .take::<Vec<FileEntry>>(0)
            .map(Ok)?
    }
}