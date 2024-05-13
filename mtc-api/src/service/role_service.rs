use axum::async_trait;

use crate::CFG;
use crate::error::api_error::ApiError;
use crate::model::role_model::{RoleCreateModel, RoleModel, RoleUpdateModel};
use crate::paginator::*;
use crate::paginator::ModelPagination;
use crate::repository::role_repository::{RoleRepository, RoleRepositoryTrait};
use crate::service_paginate;

pub struct RoleService {
    repository: RoleRepository,
}

service_paginate!(RoleService, RoleModel);
impl RoleService {
    pub fn new(repository: RoleRepository) -> Result<Self, ApiError> {
        Ok(Self { repository })
    }
}

#[async_trait]
pub trait RoleServiceTrait {
    async fn find(&self, id: &str) -> Result<RoleModel, ApiError>;
    async fn create(&self, role_create_model: RoleCreateModel) -> Result<RoleModel, ApiError>;
    async fn update(&self, id: &str, role_update_model: RoleUpdateModel) -> Result<RoleModel, ApiError>;
    async fn delete(&self, id: &str) -> Result<(), ApiError>;
}

#[async_trait]
impl RoleServiceTrait for RoleService {
    async fn find(&self, id: &str) -> Result<RoleModel, ApiError> {
        self.repository.find(id).await
    }

    async fn create(&self, role_create_model: RoleCreateModel) -> Result<RoleModel, ApiError> {
        self.repository.create(role_create_model).await
    }

    async fn update(&self, id: &str, role_update_model: RoleUpdateModel) -> Result<RoleModel, ApiError> {
        self.repository.update(id, role_update_model).await
    }

    async fn delete(&self, id: &str) -> Result<(), ApiError> {
        self.repository.delete(id).await
    }
}