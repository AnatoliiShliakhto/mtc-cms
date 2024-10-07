use super::*;

#[async_trait]
pub trait RolesRepository {
    async fn find_role_list(&self) -> Result<Vec<Entry>>;
    async fn find_custom_role_list(&self) -> Result<Vec<Entry>>;
    async fn find_roles_by_login(&self, login: Cow<'static, str>) -> Result<Vec<Cow<'static, str>>>;
    async fn find_roles_ids_by_user_id(&self, id: Cow<'static, str>) -> Result<Vec<Cow<'static, str>>>;
    async fn find_role(&self, id: Cow<'static, str>) -> Result<Role>;
    async fn update_role(&self, payload: Value, by: Cow<'static, str>) -> Result<()>;
    async fn delete_role(&self, id: Cow<'static, str>) -> Result<()>;
    async fn assign_roles_to_user(
        &self, id: Cow<'static, str>,
        roles: Vec<Cow<'static, str>>,
    ) -> Result<()>;
    async fn find_roles_max_access_level(&self, roles: &Vec<Cow<'static, str>>) -> Result<i64>;
}

#[async_trait]
impl RolesRepository for Repository {
    async fn find_role_list(&self) -> Result<Vec<Entry>> {
        let sql = r#"
            SELECT record::id(id) as id, slug, title FROM roles ORDER BY slug;
            "#;

        let roles = self.database.query(sql)
            .await?
            .take::<Vec<Entry>>(0)?;

        Ok(roles)
    }

    async fn find_custom_role_list(&self) -> Result<Vec<Entry>> {
        let sql = r#"
            SELECT record::id(id) as id, slug, title FROM roles
            WHERE slug NOT IN ["anonymous", "administrator", "writer"] ORDER BY slug;
            "#;

        let roles = self.database.query(sql)
            .await?
            .take::<Vec<Entry>>(0)?;

        Ok(roles)
    }

    async fn find_roles_by_login(
        &self,
        login: Cow<'static, str>,
    ) -> Result<Vec<Cow<'static, str>>> {
        let sql = r#"
            SELECT VALUE array::distinct(->user_roles->roles.slug) FROM users WHERE login=$login;
            "#;

        let roles = self.database.query(sql)
            .bind(("login", login))
            .await?
            .take::<Option<Vec<Cow<str>>>>(0)?
            .unwrap_or(vec![ROLE_ANONYMOUS.into()]);
        
        Ok(roles)
    }

    async fn find_roles_ids_by_user_id(&self, id: Cow<'static, str>) -> Result<Vec<Cow<'static, str>>> {
        let sql = r#"
            SELECT VALUE array::distinct(
            SELECT VALUE record::id(id) FROM ->user_roles->roles
            ) FROM type::record("users:" + $id);
            "#;

        let roles = self.database.query(sql)
            .bind(("id", id))
            .await?
            .take::<Option<Vec<Cow<str>>>>(0)?
            .unwrap_or(vec![]);

        Ok(roles)
    }

    async fn find_role(&self, id: Cow<'static, str>) -> Result<Role> {
        let sql = r#"
        SELECT *, record::id(id) as id,
        (SELECT VALUE record::id(id) FROM ->role_permissions->permissions.id) as permissions
        FROM ONLY type::record("roles:" + $id);
        "#;

        self
            .database
            .query(sql)
            .bind(("id", id))
            .await?
            .take::<Option<Role>>(0)?
            .ok_or(DatabaseError::EntryNotFound.into())
    }

    async fn update_role(&self, payload: Value, by: Cow<'static, str>) -> Result<()> {
        let mut sql = vec!["BEGIN TRANSACTION;"];
        let id =
            payload.get_str("id").unwrap_or_default();
        let slug =
            payload.get_str("slug").unwrap_or_default();
        let title =
            payload.get_str("title").unwrap_or_default();
        let user_access_level =
            payload.get_i64("user_access_level").unwrap_or(999);
        let user_access_all =
            payload.get_bool("user_access_all").unwrap_or_default();
        let permissions =
            payload.get_str_array("permissions").unwrap_or(vec![]);

        if payload.has_key("id") && !id.is_empty() {
            sql.push(r#"LET $rec_id = UPDATE type::record("roles:" + $id) MERGE {"#)
        } else {
            sql.push(r#"
            LET $rec_id = CREATE roles CONTENT {
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

        if payload.has_key("user_access_level") {
            sql.push(r#"
            user_access_level: $user_access_level,
            "#)
        }

        if payload.has_key("user_access_all") {
            sql.push(r#"
            user_access_all: $user_access_all,
            "#)
        }

        sql.push(r#"
            updated_by: $by
        };
        "#);

        sql.push("RETURN record::id($rec_id[0].id);\n");
        sql.push("COMMIT TRANSACTION;");

        let id = self
            .database
            .query(sql.concat())
            .bind(("id", id))
            .bind(("slug", slug))
            .bind(("title", title))
            .bind(("user_access_level", user_access_level))
            .bind(("user_access_all", user_access_all))
            .bind(("by", by))
            .await?
            .take::<Option<Cow<'static, str>>>(0)?;

        if let Some(id) = id {
            self.assign_permissions_to_role(
                id,
                permissions,
            ).await?
        }

        Ok(())
    }

    async fn delete_role(&self, id: Cow<'static, str>) -> Result<()> {
        let sql = r#"
            DELETE type::record("roles:" + $id)
            WHERE slug NOT IN ["anonymous", "administrator", "writer"];
        "#;

        self
            .database
            .query(sql)
            .bind(("id", id))
            .await?;

        Ok(())
    }

    async fn assign_roles_to_user(
        &self,
        id: Cow<'static, str>,
        roles: Vec<Cow<'static, str>>
    ) -> Result<()> {
        let mut sql = vec!["BEGIN TRANSACTION;"];
        let drop_roles = format!(r#"
            DELETE users:{}->user_roles;
        "#, id);
        sql.push(&drop_roles);

        let roles = roles
            .iter()
            .map(|role| format!(r#"
                RELATE users:{}->user_roles->roles:{};
            "#, id, role).into())
            .collect::<Vec<Cow<'static, str>>>().concat();
        sql.push(&roles);
        sql.push("COMMIT TRANSACTION;");

        self
            .database
            .query(sql.concat())
            .await?;

        Ok(())
    }

    async fn find_roles_max_access_level(&self, roles: &Vec<Cow<'static, str>>) -> Result<i64> {
        if roles.is_empty() { return Ok(999) }

        let mut sql = vec![r#"
            array::max(SELECT VALUE user_access_level FROM roles WHERE id in [
        "#];

        let roles_query = roles
            .iter()
            .map(|role| format!("roles:{},", role))
            .collect::<Vec<String>>()
            .concat();

        sql. push(&roles_query);
        sql.push("]);");

        let max_access_level = self
            .database
            .query(sql.concat())
            .await?
            .take::<Option<i64>>(0)?
            .unwrap_or(999);

        Ok(max_access_level)
    }
}