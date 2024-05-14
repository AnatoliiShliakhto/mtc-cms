use axum::async_trait;

use crate::CFG;
use crate::error::api_error::ApiError;
use crate::model::user_model::{UserCreateModel, UserModel, UserUpdateModel};
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
    async fn find(&self, id: &str) -> Result<UserModel, ApiError>;
    async fn find_by_login(&self, login: &str) -> Result<UserModel, ApiError>;
    async fn create(&self, user_create_model: UserCreateModel) -> Result<UserModel, ApiError>;
    async fn update(&self, id: &str, user_update_model: UserUpdateModel) -> Result<UserModel, ApiError>;
    async fn delete(&self, id: &str) -> Result<(), ApiError>;
    async fn permissions(&self, id: &str) -> Result<Vec<String>, ApiError>;
    async fn roles(&self, id: &str) -> Result<Vec<String>, ApiError>;
    async fn groups(&self, id: &str) -> Result<Vec<String>, ApiError>;
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn find(&self, id: &str) -> Result<UserModel, ApiError> {
        self.repository.find(id).await
    }

    async fn find_by_login(&self, login: &str) -> Result<UserModel, ApiError> {
        self.repository.find_by_login(login).await
    }

    async fn create(&self, user_create_model: UserCreateModel) -> Result<UserModel, ApiError> {
        self.repository.create(user_create_model).await
    }

    async fn update(&self, id: &str, user_update_model: UserUpdateModel) -> Result<UserModel, ApiError> {
        self.repository.update(id, user_update_model).await
    }

    async fn delete(&self, id: &str) -> Result<(), ApiError> {
        self.repository.delete(id).await
    }

    async fn permissions(&self, id: &str) -> Result<Vec<String>, ApiError> {
        self.repository.permissions(id).await
    }

    async fn roles(&self, id: &str) -> Result<Vec<String>, ApiError> {
        self.repository.roles(id).await
    }

    async fn groups(&self, id: &str) -> Result<Vec<String>, ApiError> {
        self.repository.groups(id).await
    }
}