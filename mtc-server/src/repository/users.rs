use super::*;

#[async_trait]
pub trait UserRepository {
    async fn find_user_list(
        &self,
        access: Access,
        login: Option<Cow<'static, str>>,
        archive: Option<bool>,
    ) -> Result<Vec<Entry>>;
    async fn find_user(&self, id: Cow<'static, str>, access: Access) -> Result<User>;
    async fn find_user_by_login(&self, login: Cow<'static, str>, access: Access) -> Result<User>;
    async fn find_user_by_api_key(&self, api_key: Cow<'static, str>) -> Result<User>;

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
    async fn check_users(
        &self,
        logins: Vec<Cow<'static, str>>,
        access: Access
    ) -> Result<Vec<UserDetailsDto>>;
    async fn find_user_state(&self, id: Cow<'static, str>) -> Result<Value>;
    async fn update_user_api_key(
        &self,
        id: Cow<'static, str>,
        session: Cow<'static, str>,
        api_key: Cow<'static, str>,
        os: Cow<'static, str>,
        device: Cow<'static, str>,
    ) -> Result<()>;
    async fn assign_api_key_to_user(
        &self,
        id: Cow<'static, str>,
        api_key_id: Cow<'static, str>
    ) -> Result<()>;
    async fn create_user_api_key(
        &self,
        id: Cow<'static, str>,
        api_key: Cow<'static, str>,
    ) -> Result<Cow<'static, str>>;
    async fn find_api_key_by_user_id(
        &self,
        id: Cow<'static, str>
    ) -> Result<Cow<'static, str>>;
}

#[async_trait]
impl UserRepository for Repository {
    async fn find_user_list(
        &self,
        access: Access,
        login: Option<Cow<'static, str>>,
        archive: Option<bool>,
    ) -> Result<Vec<Entry>> {
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
        } else if let Some(archive) = archive {
            if !archive {
                sql.push(r#"
                    AND blocked = false
                "#)
            }
        }

        let login = login.unwrap_or_default();
        if !login.is_empty() {
            sql.push(r#"
                AND login ?~ $login
            "#)
        }

        sql.push("ORDER BY login START 0 LIMIT 50;");

        let users = self
            .database
            .query(sql.concat())
            .bind(("access_level", access.level))
            .bind(("login", login))
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
            (SELECT VALUE record::id(id) FROM ->user_groups->groups)[0] ?? "" as group,
            math::max(->user_roles->roles.user_access_level) as access_level
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

    async fn find_user_by_api_key(&self, api_key: Cow<'static, str>) -> Result<User> {
        let sql = r#"
            SELECT *, record::id(id) as id,
            (SELECT VALUE record::id(id) FROM ->user_groups->groups)[0] ?? "" as group,
            math::max(->user_roles->roles.user_access_level) as access_level
            FROM users WHERE $api_key in ->user_api_keys->api_keys.api_key;
            "#;

        self.database
            .query(sql)
            .bind(("api_key", api_key))
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
        let sql = r#"
        UPDATE users SET last_access = time::now(), access_count += 1 WHERE login=$login;
        "#;

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

    async fn check_users(
        &self,
        logins: Vec<Cow<'static, str>>,
        access: Access
    ) -> Result<Vec<UserDetailsDto>> {
        let mut sql =
            vec![r#"
            SELECT record::id(id) as id, login, blocked, last_access, access_count, "" as password,
            (SELECT VALUE title FROM ->user_groups->groups)[0] ?? "" as group,
            math::max(->user_roles->roles.user_access_level) as access_level
            FROM users WHERE login in $logins AND access_level > $access_level
            "#];

        if !access.full {
            sql.push(r#"AND blocked = false"#);
        }

        Ok(self.database
            .query(sql.concat())
            .bind(("logins", logins))
            .bind(("access_level", access.level))
            .await?
            .take::<Vec<UserDetailsDto>>(0)?)
    }

    async fn find_user_state(&self, id: Cow<'static, str>) -> Result<Value> {
        let sql = r#"
            SELECT
            record::id(id) as id,
            login,
            ->user_roles->roles.slug as roles,
            array::sort(array::distinct(->user_roles->roles->role_permissions->permissions.slug)) as permissions,
            math::min(->user_roles->roles.user_access_level) as access_level,
            array::any(->user_roles->roles.user_access_all) as full_access
            FROM ONLY type::record('users:' + $user_id) WHERE blocked = false
            "#;

        self.database
            .query(sql)
            .bind(("user_id", id))
            .await?
            .take::<Option<Value>>(0)?
            .ok_or(DatabaseError::EntryNotFound.into())
    }

    async fn update_user_api_key(
        &self,
        id: Cow<'static, str>,
        api_key: Cow<'static, str>,
        session: Cow<'static, str>,
        os: Cow<'static, str>,
        device: Cow<'static, str>,
    ) -> Result<()> {
        let mut api_key_id = self.database
            .query(r#"
            RETURN (SELECT VALUE record::id(id) as id
            FROM api_keys WHERE api_key = $api_key)[0];
            "#)
            .bind(("api_key", api_key.clone()))
            .await?
            .take::<Option<Cow<'static, str>>>(0)?
            .unwrap_or_default();

        if api_key_id.is_empty()
        {
            api_key_id = self.create_user_api_key(id.clone(), api_key.clone()).await?;
        };

        let sql = r#"
        UPDATE type::record('api_keys:' + $api_key) MERGE {
            sessionid: $sessionid,
            os: $os,
            device: $device,
            is_active: true
        };
        "#;

        self.database
            .query(sql)
            .bind(("api_key", api_key_id))
            .bind(("sessionid", session))
            .bind(("os", os))
            .bind(("device", device))
            .await?;

        Ok(())
    }

    async fn assign_api_key_to_user(
        &self,
        id: Cow<'static, str>,
        api_key_id: Cow<'static, str>,
    ) -> Result<()> {
        let sql = format!(
            r#"RELATE users:{}->user_api_keys->api_keys:{};"#,
            id,
            api_key_id,
        );
        self.database
            .query(sql)
            .bind(("user_id", id))
            .await?;

        Ok(())
    }

    async fn create_user_api_key(
        &self,
        id: Cow<'static, str>,
        api_key: Cow<'static, str>,
    ) -> Result<Cow<'static, str>> {
        let sql = r#"
            BEGIN TRANSACTION;
                LET $rec_id = CREATE api_keys CONTENT {
                    api_key: $api_key
                };
                RETURN record::id($rec_id[0].id);
            COMMIT TRANSACTION;
        "#;

        let Some(api_key_id) = self.database
            .query(sql)
            .bind(("api_key", api_key))
            .await?
            .take::<Option<Cow<'static, str>>>(0)? else {
            return Err(DatabaseError::EntryNotFound.into());
        };

        self.assign_api_key_to_user(
            id.clone(),
            api_key_id.clone(),
        ).await?;

        Ok(api_key_id)
    }

    async fn find_api_key_by_user_id(&self, id: Cow<'static, str>) -> Result<Cow<'static, str>> {
        let sql = r#"
            (SELECT VALUE ->user_api_keys->(api_keys WHERE is_active = false).api_key
            FROM type::record('users:' + $id))[0][0]
        "#;
        if let Some(api_key) = self.database
            .query(sql)
            .bind(("id", id.clone()))
            .await?
            .take::<Option<Cow<'static, str>>>(0)? {
            Ok(api_key)
        } else {
            let api_key_id = self
                .create_user_api_key(id, uuid::Uuid::new_v4().to_string().into())
                .await?;
            let sql = r#"
                SELECT VALUE api_key FROM ONLY type::record('api_keys:' + $key_id);
            "#;
            self.database.query(sql)
                .bind(("key_id", api_key_id))
                .await?
                .take::<Option<Cow<'static, str>>>(0)?
                .ok_or(DatabaseError::EntryNotFound.into())
        }
    }
}