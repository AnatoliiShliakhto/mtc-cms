use axum::async_trait;

use crate::CFG;
use crate::error::api_error::ApiError;
use crate::model::user_model::UserModel;
use crate::paginator::*;
use crate::repository::user_repository::{UserRepository, UserRepositoryTrait};
use crate::service_paginate;

pub struct UserService {
    repository: UserRepository,
}

service_paginate!(UserService, UserModel);
impl UserService {
    pub fn new(repository: UserRepository) -> Result<Self, ApiError> {
        Ok(Self { repository })
    }
}

#[async_trait]
pub trait UserServiceTrait {
    async fn find(&self, login: String) -> Result<UserModel, ApiError>;
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn find(&self, login: String) -> Result<UserModel, ApiError> {
        self.repository.find(login).await
    }
}