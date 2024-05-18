use axum::async_trait;

use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::Result;
use crate::model::permission_model::{PermissionCreateModel, PermissionModel};
use crate::model::StringListModel;
use crate::service::permissions_service::PermissionsService;

#[async_trait]
pub trait PermissionsRepositoryTrait {
    async fn all(&self) -> Result<Vec<PermissionModel>>;
    async fn find(&self, id: &str) -> Result<PermissionModel>;
    async fn find_by_role(&self, role: &str) -> Result<Vec<String>>;
    async fn create(&self, model: PermissionCreateModel) -> Result<PermissionModel>;
    async fn delete(&self, id: &str) -> Result<()>;
}

#[async_trait]
impl PermissionsRepositoryTrait for PermissionsService {
    async fn all(&self) -> Result<Vec<PermissionModel>> {
        let result: Vec<PermissionModel> = self.db.query(r#"
            SELECT * FROM permissions;
            "#)
            .await?
            .take(0)?;

        Ok(result)
    }

    async fn find(
        &self,
        id: &str,
    ) -> Result<PermissionModel> {
        let result: Option<PermissionModel> = self.db.query(r#"
            SELECT * FROM type::thing('permissions', $id);
            "#)
            .bind(("id", id.to_string()))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn find_by_role(
        &self,
        role_id: &str,
    ) -> Result<Vec<String>> {
        let result: Option<StringListModel> = self.db.query(r#"
            SELECT array::distinct(->role_permissions->permissions.name) as items FROM type::thing('roles', $id);
            "#)
            .bind(("id", role_id.to_string()))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value.items),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn create(
        &self,
        model: PermissionCreateModel,
    ) -> Result<PermissionModel> {
        let result: Option<PermissionModel> = self.db.query(r#"
            CREATE permissions CONTENT {
	            name: $name,
	            title: $title
            };
            "#)
            .bind(("name", model.name))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryAlreadyExists))
        }
    }

    async fn delete(
        &self,
        id: &str,
    ) -> Result<()> {
        match self.db.query(r#"
            DELETE type::thing('permissions', $id);
            "#)
            .bind(("id", id))
            .await {
            Ok(..) => Ok(()),
            Err(e) => Err(ApiError::from(e))
        }
    }
}