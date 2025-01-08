use super::*;

pub trait PermissionsRepository {
    fn find_permission_list(&self)
        -> impl Future<Output = Result<Vec<Entry>>> + Send;
    fn find_permissions_by_login(
        &self,
        login: Cow<'static, str>,
    ) -> impl Future<Output = Result<Vec<Cow<'static, str>>>> + Send;
    fn find_custom_permissions(&self)
        -> impl Future<Output = Result<Vec<Cow<'static, str>>>> + Send;
    fn create_custom_permission(
        &self,
        permission: Cow<'static, str>,
        by: Cow<'static, str>,
    ) -> impl Future<Output = Result<()>> + Send;
    fn delete_custom_permission(&self, permission: Cow<'static, str>)
        -> impl Future<Output = Result<()>> + Send;
    fn assign_permissions_to_role(
        &self, id: Cow<'static, str>,
        permissions: Vec<Cow<'static, str>>,
    ) -> impl Future<Output = Result<()>> + Send;
}

impl PermissionsRepository for Repository {
    /// Finds all permissions in the database, ordered by slug.
    ///
    /// # Return Value
    ///
    /// A vector of [`Entry`] objects.
    ///
    /// # Errors
    ///
    /// - `GenericError::DatabaseError` if there was an error querying the database.
    async fn find_permission_list(&self) -> Result<Vec<Entry>> {
        let sql = r#"
            SELECT record::id(id) as id, slug, slug as title FROM permissions ORDER BY slug;
            "#;

        let permissions = self.database.query(sql)
            .await?
            .take::<Vec<Entry>>(0)?;

        Ok(permissions)
    }

    /// Retrieves the list of permissions associated with a user login.
    ///
    /// # Arguments
    ///
    /// * `login` - The login identifier of the user.
    ///
    /// # Returns
    ///
    /// A result containing a vector of permission slugs associated with the user.
    /// If no permissions are found, returns a default permission [`PERMISSION_PUBLIC_READ`].
    ///
    /// # Errors
    ///
    /// - `GenericError::DatabaseError` if there was an error querying the database.
    async fn find_permissions_by_login(
        &self,
        login: Cow<'static, str>,
    ) -> Result<Vec<Cow<'static, str>>> {
        let sql = r#"
            SELECT VALUE array::sort(array::distinct(->user_roles->roles->role_permissions->permissions.slug))
            FROM users WHERE login=$login;
            "#;

        let permissions = self.database.query(sql)
            .bind(("login", login))
            .await?
            .take::<Option<Vec<Cow<str>>>>(0)?
            .unwrap_or(vec![PERMISSION_PUBLIC_READ.into()]);

        Ok(permissions)
    }

    /// Retrieves the list of custom permissions defined in the system.
    ///
    /// # Returns
    ///
    /// A result containing a vector of permission slugs associated with the custom permissions.
    ///
    /// # Errors
    ///
    /// - `GenericError::DatabaseError` if there was an error querying the database.
    async fn find_custom_permissions(&self) -> Result<Vec<Cow<'static, str>>> {
        let sql = r#"
            array::distinct(SELECT VALUE string::split(slug, "::")[0]
            FROM permissions WHERE is_custom = true);
            "#;

        let permissions = self.database.query(sql)
            .await?
            .take::<Vec<Cow<str>>>(0)?;

        Ok(permissions)
    }

    /// Creates a new custom permission in the system.
    ///
    /// The `permission` argument is used as the slug for the three
    /// permissions created: `permission::read`, `permission::write`,
    /// and `permission::delete`. The `by` argument is used to set the
    /// `created_by` field for the permissions.
    ///
    /// The three permissions are also automatically assigned to the
    /// `administrator` role.
    ///
    /// # Errors
    ///
    /// - `GenericError::DatabaseError` if there was an error querying
    ///   the database.
    async fn create_custom_permission(
        &self, permission: Cow<'static, str>,
        by: Cow<'static, str>,
    ) -> Result<()> {
        let sql = [
            r#"
                BEGIN TRANSACTION;
                
                CREATE permissions CONTENT {
                    id: $permission_read_id,
                    slug: $permission_read,
                    is_custom: true,
                    created_by: $by
                };

                CREATE permissions CONTENT {
                    id: $permission_write_id,
                    slug: $permission_write,
                    is_custom: true,
                    created_by: $by
                };

                CREATE permissions CONTENT {
                    id: $permission_delete_id,
                    slug: $permission_delete,
                    is_custom: true,
                    created_by: $by
                };
            "#,
            &format!(r#"
                RELATE roles:administrator->role_permissions->permissions:{0}_read;
                RELATE roles:administrator->role_permissions->permissions:{0}_write;
                RELATE roles:administrator->role_permissions->permissions:{0}_delete;
            
                COMMIT TRANSACTION;
            "#, permission)
        ].concat();


        self.database
            .query(sql)
            .bind(("by", by))
            .bind(("permission_read_id", format!("{}_read", permission)))
            .bind(("permission_read", format!("{}::read", permission)))
            .bind(("permission_write_id", format!("{}_write", permission)))
            .bind(("permission_write", format!("{}::write", permission)))
            .bind(("permission_delete_id", format!("{}_delete", permission)))
            .bind(("permission_delete", format!("{}::delete", permission)))
            .await?;

        Ok(())
    }

    async fn delete_custom_permission(&self, permission: Cow<'static, str>) -> Result<()> {
        self.database
            .query(
                r#"
                BEGIN TRANSACTION;
                
                DELETE FROM permissions WHERE slug=$permission_read and is_custom = true;
                DELETE FROM permissions WHERE slug=$permission_write and is_custom = true;
                DELETE FROM permissions WHERE slug=$permission_delete and is_custom = true;
                
                COMMIT TRANSACTION;
                "#,
            )
            .bind(("permission_read", format!("{}::read", permission)))
            .bind(("permission_write", format!("{}::write", permission)))
            .bind(("permission_delete", format!("{}::delete", permission)))
            .await?;

        Ok(())
    }

    /// Assigns a list of permissions to a role.
    ///
    /// # Arguments
    /// * `id` - The ID of the role.
    /// * `permissions` - The list of permissions to assign to the role.
    async fn assign_permissions_to_role(
        &self,
        id: Cow<'static, str>,
        permissions: Vec<Cow<'static, str>>,
    ) -> Result<()> {
        let mut sql = vec!["BEGIN TRANSACTION;"];
        let drop_permissions = format!(r#"
            DELETE roles:{}->role_permissions;
        "#, id);
        sql.push(&drop_permissions);

        let permissions = permissions
            .iter()
            .map(|permission| format!(r#"
                RELATE roles:{}->role_permissions->permissions:{};
            "#, id, permission).into())
            .collect::<Vec<Cow<'static, str>>>().concat();
        sql.push(&permissions);
        sql.push("COMMIT TRANSACTION;");

        self
            .database
            .query(sql.concat())
            .await?;

        Ok(())
    }
}