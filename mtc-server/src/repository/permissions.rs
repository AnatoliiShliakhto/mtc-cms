use super::*;

pub trait PermissionsRepository {
    async fn find_permission_list(&self) -> Result<Vec<Entry>>;
    async fn find_permissions_by_login(
        &self,
        login: impl ToString,
    ) -> Result<Vec<Cow<'static, str>>>;
    async fn find_custom_permissions(&self) -> Result<Vec<Cow<'static, str>>>;
    async fn create_custom_permission(
        &self,
        permission: impl ToString,
        by: impl ToString,
    ) -> Result<()>;
    async fn delete_custom_permission(&self, permission: impl ToString) -> Result<()>;
    async fn assign_permissions_to_role(
        &self,
        id: impl ToString,
        permissions: Vec<Cow<'static, str>>,
    ) -> Result<()>;
}

impl PermissionsRepository for Repository {
    async fn find_permission_list(&self) -> Result<Vec<Entry>> {
        let sql = r#"
            SELECT id.id() as id, slug, slug as title FROM permissions ORDER BY slug;
            "#;

        self.database.query(sql)
            .await?
            .take::<Vec<Entry>>(0)
            .map(Ok)?
    }

    async fn find_permissions_by_login(
        &self,
        login: impl ToString,
    ) -> Result<Vec<Cow<'static, str>>> {
        let sql = r#"
            SELECT VALUE array::sort(array::distinct(->user_roles->roles->role_permissions->permissions.slug))
            FROM users WHERE login=$login;
            "#;

        self.database.query(sql)
            .bind(("login", login.to_string()))
            .await?
            .take::<Option<Vec<Cow<str>>>>(0)?
            .map_or(Ok(vec![Cow::Borrowed("public::read")]), Ok)
    }

    async fn find_custom_permissions(&self) -> Result<Vec<Cow<'static, str>>> {
        let sql = r#"
            array::distinct(SELECT VALUE string::split(slug, "::")[0]
            FROM permissions WHERE is_custom = true);
            "#;

        self.database.query(sql)
            .await?
            .take::<Vec<Cow<str>>>(0)
            .map(Ok)?
    }

    async fn create_custom_permission(
        &self,
        permission: impl ToString,
        by: impl ToString,
    ) -> Result<()> {
        let sql = r#"
            BEGIN TRANSACTION;

            LET $permission_read_rec = type::thing('permissions', $permission + '_read');
            LET $permission_write_rec = type::thing('permissions', $permission + '_write');
            LET $permission_delete_rec = type::thing('permissions', $permission + '_delete');

            CREATE $permission_read_rec CONTENT {
                slug: $permission + '::read',
                is_custom: true,
                created_by: $by
            };

            CREATE $permission_write_rec CONTENT {
                slug: $permission + '::write',
                is_custom: true,
                created_by: $by
            };

            CREATE $permission_delete_rec CONTENT {
                slug: $permission + '::delete',
                is_custom: true,
                created_by: $by
            };

            RELATE roles:administrator->role_permissions->$permission_read_rec;
            RELATE roles:administrator->role_permissions->$permission_write_rec;
            RELATE roles:administrator->role_permissions->$permission_delete_rec;

            COMMIT TRANSACTION;
        "#;


        self.database
            .query(sql)
            .bind(("by", by.to_string()))
            .bind(("permission", permission.to_string()))
            .await?
            .check()?;

        Ok(())
    }

    async fn delete_custom_permission(&self, permission: impl ToString) -> Result<()> {
        let sql = r#"
            BEGIN TRANSACTION;

            DELETE FROM permissions WHERE slug=($permission + '::read') and is_custom = true;
            DELETE FROM permissions WHERE slug=($permission + '::write') and is_custom = true;
            DELETE FROM permissions WHERE slug=($permission + '::delete') and is_custom = true;

            COMMIT TRANSACTION;
        "#;

        self.database
            .query(sql)
            .bind(("permission", permission.to_string()))
            .await?
            .check()?;

        Ok(())
    }

    async fn assign_permissions_to_role(
        &self,
        id: impl ToString,
        permissions: Vec<Cow<'static, str>>,
    ) -> Result<()> {
        let mut sql = r#"
            BEGIN TRANSACTION;
            LET $role_rec = type::thing('roles', $role_id);
            DELETE $role_rec->role_permissions;
        "#.to_string();

        let permissions = permissions
            .iter()
            .enumerate()
            .map(|(index, permission)| format!(r#"
                LET $permission_{0}_rec = type::thing('permissions', '{1}');
                RELATE $role_rec->role_permissions->$permission_{0}_rec;
            "#, index, permission).into())
            .collect::<Vec<Cow<'static, str>>>().concat();

        sql.write_str(&permissions)?;
        sql.write_str("COMMIT TRANSACTION;")?;

        self
            .database
            .query(sql)
            .bind(("role_id", id.to_string()))
            .await?
            .check()?;

        Ok(())
    }
}