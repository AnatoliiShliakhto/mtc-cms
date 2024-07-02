use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use axum::async_trait;

use mtc_model::user_model::{UserCreateModel, UserModel, UserUpdateModel};

use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::session_error::SessionError;
use crate::error::Result;
use crate::repository::RepositoryPaginate;
use crate::repository_paginate;
use crate::service::user_service::UserService;

repository_paginate!(UserService, UserModel, "users");

#[async_trait]
pub trait UserRepositoryTrait {
    async fn find_by_login(&self, login: &str) -> Result<UserModel>;
    async fn create(&self, login: &str, model: &UserCreateModel) -> Result<UserModel>;
    async fn update(&self, login: &str, model: &UserUpdateModel) -> Result<UserModel>;
    async fn delete(&self, login: &str) -> Result<()>;
    async fn role_assign(&self, user_id: &str, role_id: &str) -> Result<()>;
    async fn roles_drop(&self, user_id: &str) -> Result<()>;
    async fn group_assign(&self, user_id: &str, group_id: &str) -> Result<()>;
    async fn groups_drop(&self, user_id: &str) -> Result<()>;
    async fn block(&self, login: &str) -> Result<()>;
    async fn unblock(&self, login: &str) -> Result<()>;
    async fn block_toggle(&self, login: &str) -> Result<()>;
}

#[async_trait]
impl UserRepositoryTrait for UserService {
    async fn find_by_login(&self, login: &str) -> Result<UserModel> {
        self.db
            .query(
                r#"
                SELECT * FROM users WHERE login=$login;
                "#,
            )
            .bind(("login", login.to_string()))
            .await?
            .take::<Option<UserModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn create(&self, login: &str, model: &UserCreateModel) -> Result<UserModel> {
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

        let result: Option<UserModel> = self
            .db
            .query(
                r#"
                CREATE users CONTENT {
	                login: $login,
	                password: $password
                };
                "#,
            )
            .bind(("login", login.to_uppercase()))
            .bind(("password", password_hash))
            .await?
            .take(0)?;

        match result {
            Some(value) => {
                self.db
                    .query(format!(
                        r#"
                        RELATE users:{}->user_roles->roles:anonymous;
                        "#,
                        &value.id
                    ))
                    .await?;
                Ok(value)
            }
            _ => Err(DbError::EntryAlreadyExists.into()),
        }
    }

    async fn update(&self, login: &str, model: &UserUpdateModel) -> Result<UserModel> {
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
	                        fields: $fields
                        } WHERE login=$login;
                        "#,
                    )
                    .bind(("login", login))
                    .bind(("password", password_hash))
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
	                    fields: $fields
                    } WHERE login=$login;
                    "#,
                )
                .bind(("login", login))
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

    async fn block(&self, login: &str) -> Result<()> {
        match self
            .db
            .query(
                r#"
                    UPDATE users MERGE {
	                    blocked: true
                    } WHERE login=$login;
                    "#,
            )
            .bind(("login", login))
            .await
        {
            Ok(..) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    async fn unblock(&self, login: &str) -> Result<()> {
        match self
            .db
            .query(
                r#"
                    UPDATE users MERGE {
	                    blocked: false
                    } WHERE login=$login;
                    "#,
            )
            .bind(("login", login))
            .await
        {
            Ok(..) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    async fn block_toggle(&self, login: &str) -> Result<()> {
        let user_model = self.find_by_login(login).await?;
        match user_model.blocked {
            true => self.unblock(login).await?,
            false => self.block(login).await?,
        }
        
        Ok(())
    }
}
