use super::*;

pub trait RolesRepository {
    fn find_role_list(&self)
        -> impl Future<Output = Result<Vec<Entry>>> + Send;
    fn find_custom_role_list(&self)
        -> impl Future<Output = Result<Vec<Entry>>> + Send;
    fn find_roles_by_login(&self, login: Cow<'static, str>)
        -> impl Future<Output = Result<Vec<Cow<'static, str>>>> + Send;
    fn find_roles_ids_by_user_id(&self, id: Cow<'static, str>)
        -> impl Future<Output = Result<Vec<Cow<'static, str>>>> + Send;
    fn find_role(&self, id: Cow<'static, str>)
        -> impl Future<Output = Result<Role>> + Send;
    fn update_role(&self, payload: Value, by: Cow<'static, str>)
        -> impl Future<Output = Result<()>> + Send;
    fn delete_role(&self, id: Cow<'static, str>)
        -> impl Future<Output = Result<()>> + Send;
    fn assign_roles_to_user(
        &self, id: Cow<'static, str>,
        roles: Vec<Cow<'static, str>>,
    ) -> impl Future<Output = Result<()>> + Send;
    fn find_roles_max_access_level(&self, roles: &Vec<Cow<'static, str>>)
        -> impl Future<Output = Result<i64>> + Send;
}

impl RolesRepository for Repository {
    /// Finds all roles.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the query fails.
    ///
    /// # Returns
    ///
    /// - A JSON response containing the list of roles as a vector of [`Entry`].
    async fn find_role_list(&self) -> Result<Vec<Entry>> {
        let sql = r#"
            SELECT record::id(id) as id, slug, title FROM roles ORDER BY slug;
            "#;

        let roles = self.database.query(sql)
            .await?
            .take::<Vec<Entry>>(0)?;

        Ok(roles)
    }

    /// Finds all custom roles excluding predefined roles.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the query fails.
    ///
    /// # Returns
    ///
    /// - A JSON response containing the list of custom roles as a vector of [`Entry`].
    async fn find_custom_role_list(&self) -> Result<Vec<Entry>> {
        let sql = r#"
            SELECT record::id(id) as id, slug, title FROM roles
            WHERE slug NOT IN ["anonymous", "administrator"] ORDER BY slug;
            "#;

        let roles = self.database.query(sql)
            .await?
            .take::<Vec<Entry>>(0)?;

        Ok(roles)
    }

    /// Finds roles associated with a given user login.
    ///
    /// # Parameters
    ///
    /// - `login`: The login identifier of the user whose roles are to be retrieved.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the query fails.
    ///
    /// # Returns
    ///
    /// - A vector of role slugs associated with the user, or a vector containing
    ///   the [`ROLE_ANONYMOUS`] role if no roles are found.
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

    /// Finds the IDs of roles associated with a given user ID.
    ///
    /// # Parameters
    ///
    /// - `id`: The user ID whose roles are to be retrieved.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the query fails.
    ///
    /// # Returns
    ///
    /// - A vector of role IDs associated with the user, or an empty vector if no
    ///   roles are found.
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

    /// Finds a role by its ID.
    ///
    /// # Parameters
    ///
    /// - `id`: The ID of the role to retrieve.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the query fails.
    /// - `DatabaseError::EntryNotFound` if no role is found with the specified ID.
    ///
    /// # Returns
    ///
    /// - The [`Role`] data, if found.
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

    /// Updates a role by its ID.
    ///
    /// # Parameters
    ///
    /// - `payload`: A JSON payload containing the new values for the role.
    /// - `by`: The login identifier of the user who is updating the role.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the query fails.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the role is updated successfully.
    async fn update_role(&self, payload: Value, by: Cow<'static, str>) -> Result<()> {
        let mut sql = vec!["BEGIN TRANSACTION;"];
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
            payload.key_obj::<Vec<Cow<'static,str>>>("permissions").unwrap_or_default();

        if payload.contains_key("id") && !id.is_empty() {
            sql.push(r#"LET $rec_id = UPDATE type::record("roles:" + $id) MERGE {"#)
        } else {
            sql.push(r#"
            LET $rec_id = CREATE roles CONTENT {
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

        if payload.contains_key("user_access_level") {
            sql.push(r#"
            user_access_level: $user_access_level,
            "#)
        }

        if payload.contains_key("user_access_all") {
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

    /// Deletes a role by ID.
    ///
    /// # Note
    ///
    /// The anonymous, administrator, and writer roles are protected and cannot be deleted.
    ///
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

    /// Assigns the given roles to a user.
    ///
    /// # Parameters
    ///
    /// - `id`: The ID of the user to assign the roles to.
    /// - `roles`: A list of role IDs to assign.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the query fails.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the roles are assigned successfully.
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

    /// Finds the highest user access level among the given roles.
    ///
    /// # Parameters
    ///
    /// - `roles`: A list of role IDs to check.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the query fails.
    ///
    /// # Returns
    ///
    /// - The highest user access level found in the roles, or `999` if the list is empty.
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