use axum::async_trait;

use crate::CFG;
use crate::error::api_error::ApiError;
use crate::error::Result;
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
    pub fn new(repository: RoleRepository) -> Result<Self> { Ok(Self { repository }) }
}

#[async_trait]
pub trait RoleServiceTrait {
    async fn find(&self, id: &str) -> Result<RoleModel>;
    async fn find_by_name(&self, name: &str) -> Result<RoleModel>;
    async fn create(&self, model: RoleCreateModel) -> Result<RoleModel>;
    async fn update(&self, id: &str, model: RoleUpdateModel) -> Result<RoleModel>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn permission_assign(&self, role_id: &str, permission_id: &str) -> Result<()>;
    async fn permission_unassign(&self, role_id: &str, permission_id: &str) -> Result<()>;
}

#[async_trait]
impl RoleServiceTrait for RoleService {
    async fn find(
        &self,
        id: &str,
    ) -> Result<RoleModel> {
        self.repository.find(id).await
    }

    async fn find_by_name(
        &self,
        name: &str,
    ) -> Result<RoleModel> {
        self.repository.find_by_name(name).await
    }

    async fn create(
        &self,
        model: RoleCreateModel,
    ) -> Result<RoleModel> {
        self.repository.create(model).await
    }

    async fn update(
        &self,
        id: &str,
        model: RoleUpdateModel,
    ) -> Result<RoleModel> {
        self.repository.update(id, model).await
    }

    async fn delete(
        &self,
        id: &str,
    ) -> Result<()> {
        self.repository.delete(id).await
    }

    async fn permission_assign(&self, role_id: &str, permission_id: &str) -> Result<()> {
        match self.repository.permission_assign(role_id, permission_id).await {
            Ok(()) => Ok(()),
            Err(_) => Err(ApiError::from("Permission not assigned"))
        }
    }

    async fn permission_unassign(&self, role_id: &str, permission_id: &str) -> Result<()> {
        self.repository.permission_unassign(role_id, permission_id).await
    }
}