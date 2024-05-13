use axum::async_trait;

use crate::CFG;
use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::model::user_model::UserModel;
use crate::paginator::RepositoryPaginate;
use crate::provider::database_provider::DB;
use crate::repository_paginate;

pub struct UserRepository;

repository_paginate!(UserRepository, UserModel, "users");

#[async_trait]
pub trait UserRepositoryTrait {
    async fn find(&self, login: String) -> Result<UserModel, ApiError>;
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn find(&self, login: String) -> Result<UserModel, ApiError> {
        DB.select(("users", &login))
            .await?
            .ok_or(ApiError::from(DbError::EntryNotFound))
    }
}