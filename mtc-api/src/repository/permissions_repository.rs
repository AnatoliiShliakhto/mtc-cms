use axum::async_trait;

use crate::error::db_error::DbError;
use crate::error::Result;
use crate::model::permission_model::{PermissionModel, PermissionsModel};
use crate::service::permissions_service::PermissionsService;

#[async_trait]
pub trait PermissionsRepositoryTrait {
    async fn all(&self) -> Result<PermissionsModel>;
    async fn find_by_slug(&self, slug: &str) -> Result<PermissionModel>;
    async fn find_by_role(&self, role: &str) -> Result<PermissionsModel>;
    async fn find_by_user(&self, role: &str) -> Result<PermissionsModel>;
}

#[async_trait]
impl PermissionsRepositoryTrait for PermissionsService {
    async fn all(&self) -> Result<PermissionsModel> {
        let permissions: Vec<String> = self.db.query(r#"
            SELECT VALUE slug FROM permissions;
            "#)
            .await?
            .take(0)?;

        Ok(PermissionsModel { permissions })
    }

    async fn find_by_slug(&self, slug: &str) -> Result<PermissionModel> {
        self.db.query(r#"
            SELECT * FROM permissions WHERE slug=$slug;
            "#)
            .bind(("slug", slug))
            .await?
            .take::<Option<PermissionModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn find_by_role(
        &self,
        slug: &str,
    ) -> Result<PermissionsModel> {
        self.db.query(r#"
            SELECT array::sort(array::distinct(->role_permissions->permissions.slug)) as permissions FROM roles WHERE slug=$slug
            "#)
            .bind(("slug", slug))
            .await?
            .take::<Option<PermissionsModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn find_by_user(&self, login: &str) -> Result<PermissionsModel> {
        self.db.query(r#"
            SELECT array::sort(array::distinct(->user_roles->roles->role_permissions->permissions.slug)) as permissions
            FROM users WHERE login=$login
            "#)
            .bind(("login", login))
            .await?
            .take::<Option<PermissionsModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }
}