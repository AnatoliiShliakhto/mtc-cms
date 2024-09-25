use super::*;

#[async_trait]
pub trait UserRepository {
    async fn find_user_by_id(&self, id: Cow<'static, str>) -> Result<User>;
    async fn find_user_by_login(&self, login: Cow<'static, str>, access: Access) -> Result<User>;
    
    async fn set_user_password(
        &self, id: Cow<'static, str>,
        password_hash: Cow<'static, str>,
    ) -> Result<()>;

    async fn find_user_access(&self, login: Cow<'static, str>) -> Result<Access>;
    async fn increment_user_access_count(&self, login: Cow<'static, str>) -> Result<()>;
}

#[async_trait]
impl UserRepository for Repository {
    async fn find_user_by_id(&self, id: Cow<'static, str>) -> Result<User> {
        let sql = r#"SELECT *, record::id(id) as id FROM users WHERE id=$id;"#;
        
        self.database
            .query(sql)
            .bind(("id", id))
            .await?
            .take::<Option<User>>(0)?
            .ok_or(DatabaseError::EntryNotFound.into())
    }

    async fn find_user_by_login(&self, login: Cow<'static, str>, access: Access) -> Result<User> {
        let mut sql =
            vec![r#"SELECT *, record::id(id) as id FROM users WHERE login=$login AND access_level > $access_level"#];

        if !access.full {
            sql.push(r#"" AND blocked = false""#);
        }

        self.database
            .query(sql.concat())
            .bind(("login", login))
            .bind(("access_level", access.level))
            .await?
            .take::<Option<User>>(0)?
            .ok_or(DatabaseError::EntryNotFound.into())
    }

    async fn set_user_password(
        &self,
        id: Cow<'static, str>,
        password_hash: Cow<'static, str>
    ) -> Result<()> {
        let sql = r#"
            UPDATE users MERGE {
                password: $password
            } WHERE id=$id;
        "#;
        
        self.database.query(sql)
            .bind(("id", id))
            .bind(("password", password_hash))
            .await?;
        
        Ok(())
    }

    async fn find_user_access(&self, login: Cow<'static, str>) -> Result<Access> {
        let sql = r#"
            SELECT math::min(->user_roles->roles.user_access_level) as level,
            array::any(->user_roles->roles.user_access_all) as full
            FROM users WHERE login=$login;        
        "#;

        let access = self.database
            .query(sql)
            .bind(("login", login))
            .await?
            .take::<Option<Access>>(0)?
            .unwrap_or_default();
        
        Ok(access)
    }

    async fn increment_user_access_count(&self, login: Cow<'static, str>) -> Result<()> {
        let sql = r#"UPDATE users SET access_count += 1 WHERE login=$login;"#;

        self.database
            .query(sql)
            .bind(("login", login))
            .await?;

        Ok(())
    }
}