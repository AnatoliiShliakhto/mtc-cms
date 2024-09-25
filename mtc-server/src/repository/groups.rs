use super::*;

#[async_trait]
pub trait GroupsRepository {
    async fn find_group_list(&self) -> Result<Vec<Entry>>;
    async fn find_group_by_login(&self, login: Cow<'static, str>) -> Result<Cow<'static, str>>;
    async fn find_group(&self, id: Cow<'static, str>) -> Result<Group>;
    async fn update_group(&self, payload: Value, by: Cow<'static, str>) -> Result<()>;
    async fn delete_group(&self, id: Cow<'static, str>) -> Result<()>;
}

#[async_trait]
impl GroupsRepository for Repository {
    async fn find_group_list(&self) -> Result<Vec<Entry>> {
        let sql = r#"
        SELECT record::id(id) as id, slug, title FROM groups ORDER BY slug;
        "#;

        let groups = self
            .database
            .query(sql)
            .await?
            .take::<Vec<Entry>>(0)?;

        Ok(groups)
    }

    async fn find_group_by_login(&self, login: Cow<'static, str>) -> Result<Cow<'static, str>> {
        let sql = r#"
        array::at(SELECT VALUE ->user_groups->groups.slug FROM users WHERE login=$login, 0);
        "#;

        let group = self.database.query(sql)
            .bind(("login", login))
            .await?
            .take::<Option<Cow<str>>>(0)?
            .unwrap_or("".into());

        Ok(group)
    }

    async fn find_group(&self, id: Cow<'static, str>) -> Result<Group> {
        let sql = r#"
        SELECT *, record::id(id) as id FROM ONLY type::record("groups:" + $id);
        "#;

        self
            .database
            .query(sql)
            .bind(("id", id))
            .await?
            .take::<Option<Group>>(0)?
            .ok_or(DatabaseError::EntryNotFound.into())
    }

    async fn update_group(&self, payload: Value, by: Cow<'static, str>) -> Result<()> {
        let mut sql = vec![];
        let id = payload.get_str("id").unwrap_or_default();
        let slug = payload.get_str("slug").unwrap_or_default();
        let title = payload.get_str("title").unwrap_or_default();

        if payload.has_key("id") && !id.is_empty() {
            sql.push(r#"UPDATE type::record("groups:" + $id) MERGE {"#)
        } else {
            sql.push(r#"
            CREATE groups CONTENT {
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

        sql.push(r#"
            updated_by: $by
        };
        "#);

        self
            .database
            .query(sql.concat())
            .bind(("id", id))
            .bind(("slug", slug))
            .bind(("title", title))
            .bind(("by", by))
            .await?;

        Ok(())
    }

    async fn delete_group(&self, id: Cow<'static, str>) -> Result<()> {
        let sql = r#"
            DELETE type::record("groups:" + $id);
        "#;

        self
            .database
            .query(sql)
            .bind(("id", id))
            .await?;

        Ok(())
    }
}