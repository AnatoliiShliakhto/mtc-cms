use super::*;

#[async_trait]
pub trait UserRepository {
    async fn find_user_list(&self, access: Access) -> Result<Vec<Entry>>;
    async fn find_user(&self, id: Cow<'static, str>, access: Access) -> Result<User>;
    async fn find_user_by_login(&self, login: Cow<'static, str>, access: Access) -> Result<User>;
    
    async fn set_user_password(
        &self, id: Cow<'static, str>,
        password_hash: Cow<'static, str>,
    ) -> Result<()>;
    async fn find_user_access(&self, login: Cow<'static, str>) -> Result<Access>;
    async fn increment_user_access_count(&self, login: Cow<'static, str>) -> Result<()>;
    async fn update_user(
        &self,
        payload: Value,
        by: Cow<'static, str>,
    ) -> Result<()>;
    async fn delete_user(&self, id: Cow<'static, str>) -> Result<()>;
    async fn get_users_count(&self, only_active: bool) -> Result<i32>;
}

#[async_trait]
impl UserRepository for Repository {
    async fn find_user_list(&self, access: Access) -> Result<Vec<Entry>> {
        let mut sql = vec![r#"
            SELECT record::id(id) as id, login as slug,
            (SELECT VALUE title FROM ->user_groups->groups)[0] ?? "" as title,
            blocked as variant
            FROM users WHERE access_level > $access_level
        "#];

        if !access.full {
            sql.push(r#"
                AND blocked = false
            "#)
        }

        sql.push("ORDER BY login START 0 LIMIT 50;");

        let users = self
            .database
            .query(sql.concat())
            .await?
            .take::<Vec<Entry>>(0)?;

        Ok(users)
    }

    async fn find_user(&self, id: Cow<'static, str>, access: Access) -> Result<User> {
        let mut sql = vec![r#"
            SELECT *, record::id(id) as id,
            (SELECT VALUE record::id(id) FROM ->user_groups->groups)[0] ?? "" as group
            FROM type::record("users:" + $id) WHERE access_level > $access_level
        "#];

        if !access.full {
            sql.push(r#"AND blocked = false"#);
        }

        self.database
            .query(sql.concat())
            .bind(("id", id))
            .bind(("access_level", access.level))
            .await?
            .take::<Option<User>>(0)?
            .ok_or(DatabaseError::EntryNotFound.into())
    }

    async fn find_user_by_login(&self, login: Cow<'static, str>, access: Access) -> Result<User> {
        let mut sql =
            vec![r#"
            SELECT *, record::id(id) as id,
            (SELECT VALUE record::id(id) FROM ->user_groups->groups)[0] ?? "" as group
            FROM users WHERE login=$login AND access_level > $access_level
            "#];

        if !access.full {
            sql.push(r#"AND blocked = false"#);
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
            UPDATE type::record("users:" + $id) MERGE {
                password: $password
            };
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

    async fn update_user(
        &self,
        payload: Value,
        by: Cow<'static, str>,
    ) -> Result<()> {
        let mut sql = vec!["BEGIN TRANSACTION;"];
        let id = payload.key_str("id").unwrap_or_default();
        let login = payload.key_str("login").unwrap_or_default();
        let mut password = payload.key_str("password").unwrap_or_default();
        let group = payload.key_str("group").unwrap_or_default();
        let blocked = payload.key_bool("blocked").unwrap_or_default();
        let roles = payload.key_obj::<Vec<Cow<'static, str>>>("roles")
            .unwrap_or_default();
        let access_level = self.find_roles_max_access_level(&roles).await?;

        if !password.is_empty() {
            let Ok(salt) =
                SaltString::from_b64(&self.config.password_salt) else {
                Err(SessionError::PasswordHash)?
            };

            let argon2 = Argon2::default();
            let Ok(password_hash) = argon2.hash_password(password.as_bytes(), &salt)
            else {
                Err(SessionError::PasswordHash)?
            };
            password = password_hash.to_string().into();
        }

        if payload.contains_key("id") && !id.is_empty() {
            sql.push(r#"LET $rec_id = UPDATE type::record("users:" + $id) MERGE {"#)
        } else {
            sql.push(r#"
            LET $rec_id = CREATE users CONTENT {
                created_by: $by,
                password: $password,
            "#)
        }

        if payload.contains_key("login") {
            sql.push(r#"
            login: $login,
            "#)
        }

        if !password.is_empty() {
            sql.push(r#"
            password: $password,
            "#)
        }

        if payload.contains_key("blocked") {
            sql.push(r#"
            blocked: $blocked,
            "#)
        }

        sql.push(r#"
            access_level: $access_level,
        "#);

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
            .bind(("login", login))
            .bind(("password", password))
            .bind(("blocked", blocked))
            .bind(("access_level", access_level))
            .bind(("by", by))
            .await?
            .take::<Option<Cow<'static, str>>>(0)?;

        if let Some(id) = id {
            self.assign_group_to_user(id.clone(), group).await?;
            self.assign_roles_to_user(id, roles).await?
        }

        Ok(())
    }

    async fn delete_user(&self, id: Cow<'static, str>) -> Result<()> {
        let sql = r#"
            DELETE type::record("users:" + $id);
        "#;

        self
            .database
            .query(sql)
            .bind(("id", id))
            .await?;

        Ok(())
    }

    async fn get_users_count(&self, only_active: bool) -> Result<i32> {
        Ok(self
            .database
            .query(match only_active {
                false => r#"count(SELECT 1 FROM users);"#,
                true => r#"count(SELECT 1 FROM users WHERE blocked=false);"#,
            })
            .await?
            .take::<Option<i32>>(0)?.unwrap_or_default())
    }
}