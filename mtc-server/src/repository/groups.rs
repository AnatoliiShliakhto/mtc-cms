use super::*;

pub trait GroupsRepository {
    async fn find_group_list(&self) -> Result<Vec<Entry>>;
    async fn find_group_by_login(&self, login: impl ToString) -> Result<Cow<str>>;
    async fn find_group(&self, id: impl ToString) -> Result<Group>;
    async fn update_group(&self, payload: Value, by: impl ToString) -> Result<()>;
    async fn delete_group(&self, id: impl ToString) -> Result<()>;
    async fn assign_group_to_user(
        &self,
        id: impl ToString,
        group: impl ToString,
    ) -> Result<()>;
}

impl GroupsRepository for Repository {
    async fn find_group_list(&self) -> Result<Vec<Entry>> {
        let sql = r#"
            SELECT id.id() as id, slug, title FROM groups ORDER BY slug;
        "#;

        self
            .database
            .query(sql)
            .await?
            .take::<Vec<Entry>>(0)
            .map(Ok)?
    }

    async fn find_group_by_login(&self, login: impl ToString) -> Result<Cow<str>> {
        let sql = r#"
            array::at(SELECT VALUE ->user_groups->groups.slug FROM users WHERE login=$login, 0);
        "#;

        self.database.query(sql)
            .bind(("login", login.to_string()))
            .await?
            .take::<Option<Cow<str>>>(0)?
            .map_or(Ok(Cow::Borrowed("")), Ok)
    }

    async fn find_group(&self, id: impl ToString) -> Result<Group> {
        let sql = r#"
        SELECT *, id.id() as id FROM ONLY type::thing('groups', $group_id);
        "#;

        self
            .database
            .query(sql)
            .bind(("group_id", id.to_string()))
            .await?
            .take::<Option<Group>>(0)?
            .ok_or(DatabaseError::EntryNotFound.into())
    }

    async fn update_group(&self, payload: Value, by: impl ToString) -> Result<()> {
        let id = payload["id"].as_str().unwrap_or_default();
        let slug = payload["slug"].as_str().unwrap_or_default();
        let title = payload["title"].as_str().unwrap_or_default();

        let mut sql = String::new();

        if !id.is_empty() {
            sql.write_str(r#"UPDATE type::thing('groups', $group_id) MERGE {"#)?
        } else {
            sql.write_str(r#"
            CREATE groups CONTENT {
                created_by: $by,
            "#)?
        }

        if !slug.is_empty() {
            sql.write_str(r#"
            slug: $slug,
            "#)?
        }

        if !title.is_empty() {
            sql.write_str(r#"
            title: $title,
            "#)?
        }

        sql.write_str(r#"
            updated_by: $by
        };
        "#)?;

        self
            .database
            .query(sql)
            .bind(("group_id", id.to_string()))
            .bind(("slug", slug.to_string()))
            .bind(("title", title.to_string()))
            .bind(("by", by.to_string()))
            .await?
            .check()?;

        Ok(())
    }

    async fn delete_group(&self, id: impl ToString) -> Result<()> {
        let sql = r#"
            DELETE type::thing('groups', $group_id);
        "#;

        self
            .database
            .query(sql)
            .bind(("group_id", id.to_string()))
            .await?
            .check()?;

        Ok(())
    }


    async fn assign_group_to_user(&self, id: impl ToString, group: impl ToString) -> Result<()> {
        let id = id.to_string();
        let group = group.to_string();

        let mut sql = r#"
            BEGIN TRANSACTION;
            LET $user_rec = type::thing('users', $user_id);
            DELETE $user_rec->user_groups;
        "#.to_string();

        if !group.is_empty() {
            sql.write_str(r#"
            LET $group_rec = type::thing('groups', $group_id);
            RELATE $user_rec->user_groups->$group_rec;
            "#)?;
        }
        sql.write_str("COMMIT TRANSACTION;")?;

        self
            .database
            .query(sql)
            .bind(("user_id", id))
            .bind(("group_id", group))
            .await?
            .check()?;

        Ok(())
    }
}