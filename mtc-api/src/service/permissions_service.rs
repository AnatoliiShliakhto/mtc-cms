use axum::async_trait;

use crate::error::api_error::ApiError;
use crate::model::permission_model::PermissionModel;
use crate::repository::permissions_repository::{PermissionsRepository, PermissionsRepositoryTrait};

pub struct PermissionsService {
    repository: PermissionsRepository,
}

impl PermissionsService {
    pub fn new(repository: PermissionsRepository) -> Result<Self, ApiError> {
        Ok(Self { repository })
    }
}

#[async_trait]
pub trait PermissionsServiceTrait {
    async fn all(&self) -> Result<Vec<PermissionModel>, ApiError>;
    async fn get(&self, role: &str) -> Result<Vec<String>, ApiError>;
}

#[async_trait]
impl PermissionsServiceTrait for PermissionsService {
    async fn all(&self) -> Result<Vec<PermissionModel>, ApiError> {
        self.repository.all().await
    }

    async fn get(&self, role_id: &str) -> Result<Vec<String>, ApiError> {
        self.repository.get(role_id).await
    }
}