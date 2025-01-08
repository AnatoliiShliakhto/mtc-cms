use super::*;

pub trait UserRepository {
    fn find_user_list(
        &self,
        access: Access,
        login: Option<Cow<'static, str>>,
        archive: Option<bool>,
    ) -> impl Future<Output = Result<Vec<Entry>>> + Send;
    fn find_user(&self, id: Cow<'static, str>, access: Access)
        -> impl Future<Output = Result<User>> + Send;
    fn find_user_by_login(&self, login: Cow<'static, str>, access: Access)
        -> impl Future<Output = Result<User>> + Send;
    fn find_user_by_api_key(&self, api_key: Cow<'static, str>)
        -> impl Future<Output = Result<User>> + Send;

    fn set_user_password(
        &self, id: Cow<'static, str>,
        password_hash: Cow<'static, str>,
    ) -> impl Future<Output = Result<()>> + Send;
    fn find_user_access(&self, login: Cow<'static, str>)
        -> impl Future<Output = Result<Access>> + Send;
    fn increment_user_access_count(&self, login: Cow<'static, str>)
        -> impl Future<Output = Result<()>> + Send;
    fn update_user(
        &self,
        payload: Value,
        by: Cow<'static, str>,
    ) -> impl Future<Output = Result<()>> + Send;
    fn delete_user(&self, id: Cow<'static, str>)
        -> impl Future<Output = Result<()>> + Send;
    fn get_users_count(&self, only_active: bool)
        -> impl Future<Output = Result<i32>> + Send;
    fn check_users(
        &self,
        logins: Vec<Cow<'static, str>>,
        access: Access
    ) -> impl Future<Output = Result<Vec<UserDetailsDto>>> + Send;
    fn find_user_state(&self, id: Cow<'static, str>)
        -> impl Future<Output = Result<Value>> + Send;
    fn update_user_api_key(
        &self,
        id: Cow<'static, str>,
        session: Cow<'static, str>,
        api_key: Cow<'static, str>,
        os: Cow<'static, str>,
        device: Cow<'static, str>,
    ) -> impl Future<Output = Result<()>> + Send;
    fn assign_api_key_to_user(
        &self,
        id: Cow<'static, str>,
        api_key_id: Cow<'static, str>
    ) -> impl Future<Output = Result<()>> + Send;
    fn create_user_api_key(
        &self,
        id: Cow<'static, str>,
        api_key: Cow<'static, str>,
    ) -> impl Future<Output = Result<Cow<'static, str>>> + Send;
    fn find_api_key_by_user_id(
        &self,
        id: Cow<'static, str>
    ) -> impl Future<Output = Result<Cow<'static, str>>> + Send;
}

impl UserRepository for Repository {
    /// Return a list of users.
    ///
    /// If `login` is provided, return all users which have a login
    /// containing the given string. If `archive` is true, return archived
    /// users, otherwise return active users.
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

    /// Return the specified user.
    ///
    /// The user must exist and have a `access_level` greater than or equal to
    /// the `access_level` of the `access` parameter.
    ///
    /// If `access.full` is `false`, only active users are returned.
    ///
    /// The response is a [`User`] object.
    ///
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

    /// Return the user with the specified login.
    ///
    /// The user must exist and have a `access_level` greater than or equal to
    /// the `access_level` of the `access` parameter.
    ///
    /// If `access.full` is `false`, only active users are returned.
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

    /// Finds a user by their API key.
    ///
    /// # Parameters
    ///
    /// - `api_key`: The API key associated with the user to be retrieved.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::EntryNotFound` if no user is found with the given API key.
    ///
    /// # Returns
    ///
    /// - A [`User`] object if a user with the specified API key is found.
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

    /// Sets the password hash of a user.
    ///
    /// # Parameters
    ///
    /// - `id`: The ID of the user to modify.
    /// - `password_hash`: The new password hash.
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

    /// Finds the user access level by login.
    ///
    /// # Parameters
    ///
    /// - `login`: The login of the user to retrieve the access level for.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the query fails.
    ///
    /// # Returns
    ///
    /// - An [`Access`] struct containing the minimum access level of the user's
    ///   roles and a boolean indicating whether any of the user's roles have
    ///   full access. If no roles are found for the user, an [`Access`] struct
    ///   with `level` set to 999 and `full` set to `false` is returned.
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

    /// Increments the access count and updates the last access time for a user.
    ///
    /// # Parameters
    ///
    /// - `login`: The login identifier of the user whose access count is to be incremented.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the query fails.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the user's access count and last access time are successfully updated.
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

    /// Updates a user by their ID or creates a new user if the ID is not provided.
    ///
    /// # Parameters
    ///
    /// - `payload`: A JSON object containing user data, including fields such as `id`, `login`,
    ///   `password`, `group`, `blocked`, and `roles`.
    /// - `by`: The login identifier of the user performing the update.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the user is successfully updated or created.
    ///
    /// # Errors
    ///
    /// - `SessionError::PasswordHash` if hashing the password fails.
    /// - `DatabaseError::QueryError` if the query fails.
    ///
    /// # Notes
    ///
    /// If the `id` field is present in the payload and not empty, the function attempts to update
    /// the existing user with the given ID. Otherwise, it creates a new user. The password is
    /// hashed before storing it in the database. After updating or creating the user, the function
    /// assigns the specified group and roles to the user.
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

    /// Deletes a user by ID.
    ///
    /// # Parameters
    ///
    /// - `id`: The ID of the user to delete.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the query fails.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the user is deleted successfully.
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

    /// Check if the specified users exist and have the required access level.
    ///
    /// The function returns a list of [`UserDetailsDto`] objects, each containing the
    /// `id`, `login`, `group`, `blocked`, `last_access`, `access_count`, and `access_level`
    /// fields. If the user does not exist or does not have the required access level,
    /// the corresponding element in the list is omitted.
    ///
    /// The `access` parameter specifies the minimum access level required for the users.
    /// If `access.full` is `false`, only active users are returned.
    ///
    /// # Parameters
    ///
    /// - `logins`: A list of user logins to check.
    /// - `access`: The minimum access level required for the users.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the query fails.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<UserDetailsDto>)` if the query is successful.
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

    /// Finds the state of a user by ID.
    ///
    /// The function returns a JSON object containing the `id`, `login`, `roles`, `permissions`,
    /// `access_level`, and `full_access` fields.
    ///
    /// # Parameters
    ///
    /// - `id`: The ID of the user to find.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the query fails.
    /// - `DatabaseError::EntryNotFound` if the user is not found.
    ///
    /// # Returns
    ///
    /// - Ok([`Value`]) if the query is successful.
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

    /// Updates an API key associated with a user.
    ///
    /// The function creates the API key if it does not exist, and updates the
    /// `sessionid`, `os`, and `device` fields of the API key. The `is_active`
    /// field is set to `true`.
    ///
    /// # Parameters
    ///
    /// - `id`: The ID of the user that the API key is associated with.
    /// - `api_key`: The API key to be updated.
    /// - `session`: The session ID of the device that the API key is associated with.
    /// - `os`: The operating system of the device that the API key is associated with.
    /// - `device`: The device type of the device that the API key is associated with.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the query fails.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the query is successful.
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

    /// Assigns an API key to a user.
    ///
    /// # Parameters
    ///
    /// - `id`: The ID of the user to whom the API key will be assigned.
    /// - `api_key_id`: The ID of the API key to assign to the user.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the database query fails.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the API key is successfully assigned to the user.
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

    /// Creates a new API key for a user and assigns it to them.
    ///
    /// # Parameters
    ///
    /// - `id`: The ID of the user to whom the API key should be assigned.
    /// - `api_key`: The API key to create and assign.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the database query fails.
    /// - `DatabaseError::EntryNotFound` if the API key could not be created.
    ///
    /// # Returns
    ///
    /// - `Ok(api_key_id)` if the API key is successfully created and assigned to the user.
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

    /// Finds an API key associated with a user by the user ID.
    ///
    /// The function searches for an inactive API key associated with the given
    /// user ID. If one is found, it is returned. If not, a new API key is created
    /// and assigned to the user, and then returned.
    ///
    /// # Parameters
    ///
    /// - `id`: The ID of the user to find the API key for.
    ///
    /// # Errors
    ///
    /// - `DatabaseError::QueryError` if the database query fails.
    /// - `DatabaseError::EntryNotFound` if no API key can be found or created.
    ///
    /// # Returns
    ///
    /// - `Ok(api_key)` if the API key is found or created successfully.
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