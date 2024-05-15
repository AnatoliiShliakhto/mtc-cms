use axum::async_trait;

use crate::error::Result;
use crate::model::permission_model::PermissionModel;
use crate::repository::permissions_repository::{PermissionsRepository, PermissionsRepositoryTrait};

pub struct PermissionsService {
    repository: PermissionsRepository,
}

impl PermissionsService {
    pub fn new(repository: PermissionsRepository) -> Result<Self> {
        Ok(Self { repository })
    }
}

#[async_trait]
pub trait PermissionsServiceTrait {
    async fn all(&self) -> Result<Vec<PermissionModel>>;
    async fn find_by_role(&self, id: &str) -> Result<Vec<String>>;
}

#[async_trait]
impl PermissionsServiceTrait for PermissionsService {
    async fn all(&self) -> Result<Vec<PermissionModel>> {
        self.repository.all().await
    }

    async fn find_by_role(
        &self,
        id: &str,
    ) -> Result<Vec<String>> {
        self.repository.find_by_role(id).await
    }
}