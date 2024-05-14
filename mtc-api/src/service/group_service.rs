use axum::async_trait;

use crate::CFG;
use crate::error::api_error::ApiError;
use crate::model::group_model::{GroupCreateModel, GroupModel, GroupUpdateModel};
use crate::paginator::*;
use crate::paginator::ModelPagination;
use crate::repository::group_repository::{GroupRepository, GroupRepositoryTrait};
use crate::service_paginate;

pub struct GroupService {
    repository: GroupRepository,
}

service_paginate!(GroupService, GroupModel);
impl GroupService {
    pub fn new(repository: GroupRepository) -> Result<Self, ApiError> {
        Ok(Self { repository })
    }
}

#[async_trait]
pub trait GroupServiceTrait {
    async fn find(&self, id: &str) -> Result<GroupModel, ApiError>;
    async fn create(&self, group_create_model: GroupCreateModel) -> Result<GroupModel, ApiError>;
    async fn update(&self, id: &str, group_update_model: GroupUpdateModel) -> Result<GroupModel, ApiError>;
    async fn delete(&self, id: &str) -> Result<(), ApiError>;
}

#[async_trait]
impl GroupServiceTrait for GroupService {
    async fn find(&self, id: &str) -> Result<GroupModel, ApiError> {
        self.repository.find(id).await
    }

    async fn create(&self, group_create_model: GroupCreateModel) -> Result<GroupModel, ApiError> {
        self.repository.create(group_create_model).await
    }

    async fn update(&self, id: &str, group_update_model: GroupUpdateModel) -> Result<GroupModel, ApiError> {
        self.repository.update(id, group_update_model).await
    }

    async fn delete(&self, id: &str) -> Result<(), ApiError> {
        self.repository.delete(id).await
    }
}