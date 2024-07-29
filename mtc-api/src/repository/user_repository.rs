use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use axum::async_trait;
use surrealdb::sql::Datetime;

use mtc_model::user_model::{UserCreateModel, UserModel, UserUpdateModel};

use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::session_error::SessionError;
use crate::error::Result;
use crate::model::access_model::AccessModel;
use crate::service::user_service::UserService;

#[async_trait]
pub trait UserRepositoryTrait {
    async fn get_page(
        &self,
        start: usize,
        limit: usize,
        access: &AccessModel,
    ) -> Result<Vec<UserModel>>;
    async fn get_total(&self, access: &AccessModel) -> Result<usize>;
    async fn find_by_login(&self, login: &str, access: &AccessModel) -> Result<UserModel>;
    async fn create(&self, auth: &str, login: &str, model: &UserCreateModel) -> Result<UserModel>;
    async fn update(&self, auth: &str, login: &str, model: &UserUpdateModel) -> Result<UserModel>;
    async fn delete(&self, login: &str) -> Result<()>;
    async fn role_assign(&self, user_id: &str, role_id: &str) -> Result<()>;
    async fn roles_drop(&self, user_id: &str) -> Result<()>;
    async fn group_assign(&self, user_id: &str, group_id: &str) -> Result<()>;
    async fn groups_drop(&self, user_id: &str) -> Result<()>;
    async fn block(&self, auth: &str, login: &str) -> Result<()>;
    async fn unblock(&self, auth: &str, login: &str) -> Result<()>;
    async fn change_password(&self, login: &str, password: &str) -> Result<UserModel>;
    async fn update_access(&self, login: &str, access_count: i32) -> Result<()>;
    async fn get_roles_min_access_level(&self, login: &str) -> Result<i32>;
    async fn get_roles_max_access_level(&self, login: &str) -> Result<i32>;
    async fn update_access_level(&self, login: &str, access_level: i32) -> Result<()>;
    async fn get_roles_access_all(&self, login: &str) -> Result<bool>;
}

#[async_trait]
impl UserRepositoryTrait for UserService {
    async fn get_page(
        &self,
        start: usize,
        limit: usize,
        access: &AccessModel,
    ) -> Result<Vec<UserModel>> {
        let mut blocked_sql = "";
        if !access.users_all {
            blocked_sql = "AND blocked = false ";
        }
        let mut result = self
            .db
            .query(
                [
                    r#"
            SELECT * FROM users WHERE access_level > $access_level "#,
                    blocked_sql,
                    r#"ORDER BY updated_at DESC LIMIT $limit START $start;"#,
                ]
                .concat(),
            )
            .bind(("start", start - 1))
            .bind(("limit", limit))
            .bind(("access_level", access.users_level))
            .await?;
        let result: Vec<UserModel> = result.take(0)?;
        Ok(result)
    }

    async fn get_total(&self, access: &AccessModel) -> Result<usize> {
        let mut blocked_sql = "";
        if !access.users_all {
            blocked_sql = "AND blocked = false ";
        }
        let mut result = self
            .db
            .query(
                [
                    r#"
            SELECT count() FROM users WHERE access_level > $access_level "#,
                    blocked_sql,
                    r#"GROUP ALL;"#,
                ]
                .concat(),
            )
            .bind(("access_level", access.users_level))
            .await?;
        let result: Option<mtc_model::pagination_model::CountModel> = result.take(0)?;
        match result {
            Some(value) => Ok(value.count),
            _ => Ok(0usize),
        }
    }

    async fn find_by_login(&self, login: &str, access: &AccessModel) -> Result<UserModel> {
        let mut blocked_sql = "";
        if !access.users_all {
            blocked_sql = " AND blocked = false";
        }
        self.db
            .query(
                [
                    r#"SELECT * FROM users WHERE login=$login AND access_level > $access_level"#,
                    blocked_sql,
                ]
                .concat(),
            )
            .bind(("login", login.to_string()))
            .await?
            .take::<Option<UserModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn create(&self, auth: &str, login: &str, model: &UserCreateModel) -> Result<UserModel> {
        let password = model.password.as_bytes();
        let salt = match SaltString::from_b64(&self.cfg.password_salt) {
            Ok(value) => value,
            _ => Err(ApiError::from(SessionError::PasswordHash))?,
        };

        let argon2 = Argon2::default();
        let password_hash = match argon2.hash_password(password, &salt) {
            Ok(value) => value.to_string(),
            _ => Err(ApiError::from(SessionError::PasswordHash))?,
        };

        self.db
            .query(
                r#"
                CREATE users CONTENT {
	                login: $login,
	                password: $password,
	                blocked: $blocked,
	                created_by: $auth_id,
	                updated_by: $auth_id
                };
                "#,
            )
            .bind(("auth_id", auth))
            .bind(("login", login.to_uppercase()))
            .bind(("password", password_hash))
            .bind(("blocked", model.blocked))
            .await?
            .take::<Option<UserModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn update(&self, auth: &str, login: &str, model: &UserUpdateModel) -> Result<UserModel> {
        match &model.password {
            Some(value) => {
                let password = value.as_bytes();
                let salt = match SaltString::from_b64(&self.cfg.password_salt) {
                    Ok(value) => value,
                    _ => Err(ApiError::from(SessionError::PasswordHash))?,
                };

                let argon2 = Argon2::default();
                let password_hash = match argon2.hash_password(password, &salt) {
                    Ok(value) => value.to_string(),
                    _ => Err(ApiError::from(SessionError::PasswordHash))?,
                };
                self.db
                    .query(
                        r#"
                        UPDATE users MERGE {
                            password: $password,
                            blocked: $blocked,
	                        fields: $fields,
	                        updated_by: $auth_id
                        } WHERE login=$login;
                        "#,
                    )
                    .bind(("auth_id", auth))
                    .bind(("login", login))
                    .bind(("password", password_hash))
                    .bind(("blocked", model.blocked))
                    .bind(("fields", model.fields.clone()))
                    .await?
                    .take::<Option<UserModel>>(0)?
                    .ok_or(DbError::EntryNotFound.into())
            }
            None => self
                .db
                .query(
                    r#"
                    UPDATE users MERGE {
                        blocked: $blocked,
	                    fields: $fields,
	                    updated_by: $auth_id
                    } WHERE login=$login;
                    "#,
                )
                .bind(("auth_id", auth))
                .bind(("login", login))
                .bind(("blocked", model.blocked))
                .bind(("fields", model.fields.clone()))
                .await?
                .take::<Option<UserModel>>(0)?
                .ok_or(DbError::EntryNotFound.into()),
        }
    }

    async fn delete(&self, login: &str) -> Result<()> {
        match self
            .db
            .query(
                r#"
                DELETE FROM users WHERE login=$login;
                "#,
            )
            .bind(("login", login))
            .await
        {
            Ok(..) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
    async fn role_assign(&self, user_id: &str, role_id: &str) -> Result<()> {
        match self
            .db
            .query(format!(
                r#"
                RELATE users:{}->user_roles->roles:{};
                "#,
                user_id, role_id
            ))
            .await
        {
            Ok(..) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    async fn roles_drop(&self, user_id: &str) -> Result<()> {
        match self
            .db
            .query(
                r#"
                DELETE type::thing('users', $user_id)->user_roles;
                "#,
            )
            .bind(("user_id", user_id))
            .await
        {
            Ok(..) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    async fn group_assign(&self, user_id: &str, group_id: &str) -> Result<()> {
        match self
            .db
            .query(format!(
                r#"
                RELATE users:{}->user_groups->groups:{};
                "#,
                user_id, group_id
            ))
            .await
        {
            Ok(..) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    async fn groups_drop(&self, user_id: &str) -> Result<()> {
        match self
            .db
            .query(
                r#"
                DELETE type::thing('users', $user_id)->user_groups;
                "#,
            )
            .bind(("user_id", user_id))
            .await
        {
            Ok(..) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    async fn block(&self, auth: &str, login: &str) -> Result<()> {
        match self
            .db
            .query(
                r#"
                    UPDATE users MERGE {
	                    blocked: true,
	                    updated_by: $auth_id
                    } WHERE login=$login;
                    "#,
            )
            .bind(("auth_id", auth))
            .bind(("login", login))
            .await
        {
            Ok(..) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    async fn unblock(&self, auth: &str, login: &str) -> Result<()> {
        match self
            .db
            .query(
                r#"
                    UPDATE users MERGE {
	                    blocked: false,
	                    updated_by: $auth_id
                    } WHERE login=$login;
                    "#,
            )
            .bind(("auth_id", auth))
            .bind(("login", login))
            .await
        {
            Ok(..) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    async fn change_password(&self, login: &str, password: &str) -> Result<UserModel> {
        let password = password.as_bytes();
        let salt = match SaltString::from_b64(&self.cfg.password_salt) {
            Ok(value) => value,
            _ => Err(ApiError::from(SessionError::PasswordHash))?,
        };

        let argon2 = Argon2::default();
        let password_hash = match argon2.hash_password(password, &salt) {
            Ok(value) => value.to_string(),
            _ => Err(ApiError::from(SessionError::PasswordHash))?,
        };

        self.db
            .query(
                r#"
                UPDATE users MERGE {
                    password: $password,
                } WHERE login=$login;
            "#,
            )
            .bind(("login", login))
            .bind(("password", password_hash))
            .await?
            .take::<Option<UserModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn update_access(&self, login: &str, access_count: i32) -> Result<()> {
        self.db
            .query(
                r#"
                UPDATE users MERGE {
                    access_count: $access_count,
                    last_access: $last_access,
                } WHERE login=$login;
            "#,
            )
            .bind(("login", login))
            .bind(("access_count", access_count))
            .bind(("last_access", Datetime::default()))
            .await?;

        Ok(())
    }

    async fn get_roles_min_access_level(&self, login: &str) -> Result<i32> {
        Ok(self.db.query(r#"
            SELECT VALUE math::min(->user_roles->roles.user_access_level) FROM users WHERE login=$login
            "#)
            .bind(("login", login))
            .await?
            .take::<Option<i32>>(0)?
            .unwrap_or(999))
    }

    async fn get_roles_max_access_level(&self, login: &str) -> Result<i32> {
        Ok(self.db.query(r#"
            SELECT VALUE math::max(->user_roles->roles.user_access_level) FROM users WHERE login=$login
            "#)
            .bind(("login", login))
            .await?
            .take::<Option<i32>>(0)?
            .unwrap_or(999))
    }

    async fn update_access_level(&self, login: &str, access_level: i32) -> Result<()> {
        self.db
            .query(
                r#"
                UPDATE users MERGE {
                    access_level: $access_level
                } WHERE login=$login;
            "#,
            )
            .bind(("login", login))
            .bind(("access_level", access_level))
            .await?;

        Ok(())
    }

    async fn get_roles_access_all(&self, login: &str) -> Result<bool> {
        let access = self.db.query(r#"
            SELECT VALUE array::distinct(->user_roles->roles.user_access_all) FROM users WHERE login=$login
            "#)
            .bind(("login", login))
            .await?
            .take::<Option<Vec<bool>>>(0)?;

        Ok(access.unwrap_or(vec![false]).iter().any(|value| *value))
    }
}
