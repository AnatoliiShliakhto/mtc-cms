use axum::async_trait;

use crate::CFG;
use crate::error::Result;
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
    pub fn new(repository: GroupRepository) -> Result<Self> {
        Ok(Self { repository })
    }
}

#[async_trait]
pub trait GroupServiceTrait {
    async fn find(&self, id: &str) -> Result<GroupModel>;
    async fn create(&self, model: GroupCreateModel) -> Result<GroupModel>;
    async fn update(&self, id: &str, model: GroupUpdateModel) -> Result<GroupModel>;
    async fn delete(&self, id: &str) -> Result<()>;
}

#[async_trait]
impl GroupServiceTrait for GroupService {
    async fn find(
        &self,
        id: &str,
    ) -> Result<GroupModel> {
        self.repository.find(id).await
    }

    async fn create(
        &self,
        model: GroupCreateModel,
    ) -> Result<GroupModel> {
        self.repository.create(model).await
    }

    async fn update(
        &self,
        id: &str,
        model: GroupUpdateModel,
    ) -> Result<GroupModel> {
        self.repository.update(id, model).await
    }

    async fn delete(
        &self,
        id: &str,
    ) -> Result<()> {
        self.repository.delete(id).await
    }
}