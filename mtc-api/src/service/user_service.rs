use axum::async_trait;

use crate::CFG;
use crate::error::api_error::ApiError;
use crate::error::Result;
use crate::model::user_model::{UserCreateModel, UserModel, UserUpdateModel};
use crate::paginator::*;
use crate::repository::user_repository::{UserRepository, UserRepositoryTrait};
use crate::service_paginate;

pub struct UserService {
    repository: UserRepository,
}

service_paginate!(UserService, UserModel);

impl UserService {
    pub fn new(repository: UserRepository) -> Result<Self> {
        Ok(Self { repository })
    }
}

#[async_trait]
pub trait UserServiceTrait {
    async fn find(&self, id: &str) -> Result<UserModel>;
    async fn find_by_login(&self, login: &str) -> Result<UserModel>;
    async fn create(&self, model: UserCreateModel) -> Result<UserModel>;
    async fn update(&self, id: &str, model: UserUpdateModel) -> Result<UserModel>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn permissions(&self, id: &str) -> Result<Vec<String>>;
    async fn roles(&self, id: &str) -> Result<Vec<String>>;
    async fn groups(&self, id: &str) -> Result<Vec<String>>;
    async fn role_assign(&self, user_id: &str, role_id: &str) -> Result<()>;
    async fn role_unassign(&self, user_id: &str, role_id: &str) -> Result<()>;
    async fn group_assign(&self, user_id: &str, group_id: &str) -> Result<()>;
    async fn group_unassign(&self, user_id: &str, group_id: &str) -> Result<()>;
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn find(
        &self,
        id: &str,
    ) -> Result<UserModel> {
        self.repository.find(id).await
    }

    async fn find_by_login(
        &self,
        login: &str,
    ) -> Result<UserModel> {
        self.repository.find_by_login(login).await
    }

    async fn create(
        &self,
        model: UserCreateModel,
    ) -> Result<UserModel> {
        self.repository.create(model).await
    }

    async fn update(
        &self,
        id: &str,
        model: UserUpdateModel,
    ) -> Result<UserModel> {
        self.repository.update(id, model).await
    }

    async fn delete(
        &self,
        id: &str,
    ) -> Result<()> {
        self.repository.delete(id).await
    }

    async fn permissions(
        &self,
        id: &str,
    ) -> Result<Vec<String>> {
        self.repository.permissions(id).await
    }

    async fn roles(
        &self,
        id: &str,
    ) -> Result<Vec<String>> {
        self.repository.roles(id).await
    }

    async fn groups(
        &self,
        id: &str,
    ) -> Result<Vec<String>> {
        self.repository.groups(id).await
    }

    async fn role_assign(
        &self,
        user_id: &str,
        role_id: &str,
    ) -> Result<()> {
        match self.repository.role_assign(user_id, role_id).await {
            Ok(()) => Ok(()),
            Err(_) => Err(ApiError::from("Role not assigned"))
        }
    }

    async fn role_unassign(
        &self,
        user_id: &str,
        role_id: &str,
    ) -> Result<()> {
        self.repository.role_unassign(user_id, role_id).await
    }

    async fn group_assign(&self, user_id: &str, group_id: &str) -> Result<()> {
        match self.repository.group_assign(user_id, group_id).await {
            Ok(()) => Ok(()),
            Err(_) => Err(ApiError::from("Group not assigned"))
        }
    }

    async fn group_unassign(&self, user_id: &str, group_id: &str) -> Result<()> {
        self.repository.group_unassign(user_id, group_id).await
    }
}