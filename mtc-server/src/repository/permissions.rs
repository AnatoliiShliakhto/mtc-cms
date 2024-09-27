use super::*;

#[async_trait]
pub trait PermissionsRepository {
    async fn find_permission_list(&self) -> Result<Vec<Entry>>;
    async fn find_permissions_by_login(
        &self,
        login: Cow<'static, str>,
    ) -> Result<Vec<Cow<'static, str>>>;
    async fn find_custom_permissions(&self) -> Result<Vec<Cow<'static, str>>>;
    async fn create_custom_permission(
        &self,
        permission: Cow<'static, str>,
        creator: Cow<'static, str>,
    ) -> Result<()>;
    async fn delete_custom_permission(&self, permission: Cow<'static, str>) -> Result<()>;
    async fn assign_permissions_to_role(
        &self, id: Cow<'static, str>,
        permissions: Vec<Cow<'static, str>>,
    ) -> Result<()>;
}

#[async_trait]
impl PermissionsRepository for Repository {
    async fn find_permission_list(&self) -> Result<Vec<Entry>> {
        let sql = r#"
            SELECT record::id(id) as id, slug, slug as title FROM permissions ORDER BY slug;
            "#;

        let permissions = self.database.query(sql)
            .await?
            .take::<Vec<Entry>>(0)?;

        Ok(permissions)
    }

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

    async fn create_custom_permission(
        &self, permission: Cow<'static, str>,
        creator: Cow<'static, str>,
    ) -> Result<()> {
        let sql = [
            r#"
                BEGIN TRANSACTION;
                
                CREATE permissions CONTENT {
                    id: $permission_read_id,
                    slug: $permission_read,
                    is_custom: true,
                    created_by: $creator
                };

                CREATE permissions CONTENT {
                    id: $permission_write_id,
                    slug: $permission_write,
                    is_custom: true,
                    created_by: $creator
                };

                CREATE permissions CONTENT {
                    id: $permission_delete_id,
                    slug: $permission_delete,
                    is_custom: true,
                    created_by: $creator
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
            .bind(("creator", creator))
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

    async fn assign_permissions_to_role(&self, id: Cow<'static, str>, permissions: Vec<Cow<'static, str>>) -> Result<()> {
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