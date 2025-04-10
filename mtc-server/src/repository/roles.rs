use super::*;

pub trait RolesRepository {
    async fn find_role_list(&self) -> Result<Vec<Entry>>;
    async fn find_custom_role_list(&self) -> Result<Vec<Entry>>;
    async fn find_roles_by_login(&self, login: impl ToString) -> Result<Vec<Cow<'static, str>>>;
    async fn find_roles_ids_by_user_id(&self, id: impl ToString) -> Result<Vec<Cow<'static, str>>>;
    async fn find_role(&self, id: impl ToString) -> Result<Role>;
    async fn update_role(&self, payload: Value, by: impl ToString) -> Result<()>;
    async fn delete_role(&self, id: impl ToString) -> Result<()>;
    async fn assign_roles_to_user(
        &self, id: impl ToString,
        roles: Vec<Cow<'static, str>>,
    ) -> Result<()>;
    async fn find_roles_max_access_level(&self, roles: &Vec<Cow<'static, str>>) -> Result<i64>;
}

impl RolesRepository for Repository {
    async fn find_role_list(&self) -> Result<Vec<Entry>> {
        let sql = r#"
            SELECT id.id() as id, slug, title FROM roles ORDER BY slug;
            "#;

        self.database.query(sql)
            .await?
            .take::<Vec<Entry>>(0)
            .map(Ok)?
    }

    async fn find_custom_role_list(&self) -> Result<Vec<Entry>> {
        let sql = r#"
            SELECT id.id() as id, slug, title FROM roles
            WHERE slug NOT IN ['anonymous', 'administrator'] ORDER BY slug;
            "#;

        self.database.query(sql)
            .await?
            .take::<Vec<Entry>>(0)
            .map(Ok)?
    }

    async fn find_roles_by_login(
        &self,
        login: impl ToString,
    ) -> Result<Vec<Cow<'static, str>>> {
        let sql = r#"
            SELECT VALUE array::distinct(->user_roles->roles.slug) FROM users WHERE login=$login;
            "#;

        self.database.query(sql)
            .bind(("login", login.to_string()))
            .await?
            .take::<Option<Vec<Cow<str>>>>(0)?
            .map_or(Ok(vec![ROLE_ANONYMOUS.into()]), Ok)
    }

    async fn find_roles_ids_by_user_id(&self, id: impl ToString) -> Result<Vec<Cow<'static, str>>> {
        let sql = r#"
            SELECT VALUE array::distinct(
            SELECT VALUE id.id() FROM ->user_roles->roles
            ) FROM type::thing('users', $id);
            "#;

        self.database.query(sql)
            .bind(("id", id.to_string()))
            .await?
            .take::<Option<Vec<Cow<str>>>>(0)?
            .map_or(Ok(Vec::new()), Ok)
    }

    async fn find_role(&self, id: impl ToString) -> Result<Role> {
        let sql = r#"
        SELECT *, id.id() as id,
        (SELECT VALUE id.id() FROM ->role_permissions->permissions.id) as permissions
        FROM ONLY type::thing('roles', $id);
        "#;

        self
            .database
            .query(sql)
            .bind(("id", id.to_string()))
            .await?
            .take::<Option<Role>>(0)?
            .ok_or(DatabaseError::EntryNotFound.into())
    }

    async fn update_role(&self, payload: Value, by: impl ToString) -> Result<()> {
        let mut sql = "BEGIN TRANSACTION;".to_string();
        let id =
            payload.key_str("id").unwrap_or_default();
        let slug =
            payload.key_str("slug").unwrap_or_default();
        let title =
            payload.key_str("title").unwrap_or_default();
        let user_access_level =
            payload.key_i64("user_access_level").unwrap_or(999);
        let user_access_all =
            payload.key_bool("user_access_all").unwrap_or_default();
        let permissions =
            payload.key_obj::<Vec<Cow<'static, str>>>("permissions").unwrap_or_default();

        if payload.contains_key("id") && !id.is_empty() {
            sql.write_str(r#"LET $rec_id = UPDATE type::thing('roles', $id) MERGE {"#)?
        } else {
            sql.write_str(r#"
            LET $rec_id = CREATE roles CONTENT {
                created_by: $by,
            "#)?
        }

        if payload.contains_key("slug") {
            sql.write_str(r#"slug: $slug,"#)?
        }

        if payload.contains_key("title") {
            sql.write_str(r#"title: $title,"#)?
        }

        if payload.contains_key("user_access_level") {
            sql.write_str(r#"user_access_level: $user_access_level,"#)?
        }

        if payload.contains_key("user_access_all") {
            sql.write_str(r#"user_access_all: $user_access_all,"#)?
        }

        sql.write_str(r#"updated_by: $by};"#)?;

        sql.write_str("RETURN $rec_id[0].id.id(); COMMIT TRANSACTION;")?;

        let id = self
            .database
            .query(sql)
            .bind(("id", id))
            .bind(("slug", slug))
            .bind(("title", title))
            .bind(("user_access_level", user_access_level))
            .bind(("user_access_all", user_access_all))
            .bind(("by", by.to_string()))
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

    async fn delete_role(&self, id: impl ToString) -> Result<()> {
        let sql = r#"
            DELETE type::thing('roles', $id)
            WHERE slug NOT IN ['anonymous', 'administrator', 'writer'];
        "#;

        self
            .database
            .query(sql)
            .bind(("id", id.to_string()))
            .await?
            .check()?;

        Ok(())
    }

    async fn assign_roles_to_user(
        &self,
        id: impl ToString,
        roles: Vec<Cow<'static, str>>,
    ) -> Result<()> {
        let mut sql = r#"
            BEGIN TRANSACTION;
            LET $user_rec = type::thing('users', $user_id);
            DELETE $user_rec->user_roles;
            "#.to_string();

        let roles = roles
            .iter()
            .enumerate()
            .map(|(index, role)| format!(r#"
                LET $role_{0}_rec = type::thing('roles', '{1}');
                RELATE $user_rec->user_roles->$role_{0}_rec;
            "#, index, role).into())
            .collect::<Vec<Cow<'static, str>>>().concat();

        sql.write_str(&roles)?;
        sql.write_str("COMMIT TRANSACTION;")?;

        self
            .database
            .query(sql)
            .bind(("user_id", id.to_string()))
            .await?
            .check()?;

        Ok(())
    }

    async fn find_roles_max_access_level(&self, roles: &Vec<Cow<'static, str>>) -> Result<i64> {
        if roles.is_empty() { return Ok(999); }

        let mut sql = r#"
            array::max(SELECT VALUE user_access_level FROM roles WHERE id in [
        "#.to_string();

        let roles_query = roles
            .iter()
            .map(|role| format!("roles:{},", role))
            .collect::<Vec<String>>()
            .concat();

        sql.write_str(&roles_query)?;
        sql.write_str("]);")?;

        self
            .database
            .query(sql)
            .await?
            .take::<Option<i64>>(0)?
            .map_or(Ok(999), Ok)
    }
}