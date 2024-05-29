use axum::async_trait;

use crate::error::db_error::DbError;
use crate::error::Result;
use crate::model::role_model::{RoleCreateModel, RoleModel, RolesModel, RoleUpdateModel};
use crate::repository::RepositoryPaginate;
use crate::repository_paginate;
use crate::service::role_service::RoleService;

repository_paginate!(RoleService, RoleModel, "roles");

#[async_trait]
pub trait RoleRepositoryTrait {
    async fn find_by_slug(&self, slug: &str) -> Result<RoleModel>;
    async fn find_by_user(&self, login: &str) -> Result<RolesModel>;
    async fn create(&self, slug: &str, model: RoleCreateModel) -> Result<RoleModel>;
    async fn update(&self, slug: &str, model: RoleUpdateModel) -> Result<RoleModel>;
    async fn delete(&self, slug: &str) -> Result<()>;
    async fn permission_assign(&self, role_id: &str, permission_id: &str) -> Result<()>;
    async fn permissions_drop(&self, role_id: &str) -> Result<()>;
}

#[async_trait]
impl RoleRepositoryTrait for RoleService {
    async fn find_by_slug(
        &self,
        slug: &str,
    ) -> Result<RoleModel> {
        self.db.query(r#"
            SELECT * FROM roles WHERE slug=$slug;
            "#)
            .bind(("slug", slug))
            .await?
            .take::<Option<RoleModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn find_by_user(&self, login: &str) -> Result<RolesModel> {
        self.db.query(r#"
            SELECT array::sort(array::distinct(->user_roles->roles.slug)) as roles FROM users WHERE login=$login
            "#)
            .bind(("login", login))
            .await?
            .take::<Option<RolesModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn create(
        &self,
        slug: &str,
        model: RoleCreateModel,
    ) -> Result<RoleModel> {
        self.db.query(r#"
            CREATE roles CONTENT {
	            slug: $slug,
	            title: $title
            };
            "#)
            .bind(("slug", slug))
            .bind(("title", model.title))
            .await?
            .take::<Option<RoleModel>>(0)?
            .ok_or(DbError::EntryAlreadyExists.into())
    }

    async fn update(
        &self,
        slug: &str,
        model: RoleUpdateModel,
    ) -> Result<RoleModel> {
        self.db.query(r#"
            UPDATE roles MERGE {
	            title: $title
            } WHERE slug=$slug;
            "#)
            .bind(("slug", slug))
            .bind(("title", model.title))
            .await?
            .take::<Option<RoleModel>>(0)?
            .ok_or(DbError::EntryUpdate.into())
    }

    async fn delete(
        &self,
        slug: &str,
    ) -> Result<()> {
        self.db.query(r#"
            DELETE FROM roles WHERE slug=$slug;
            "#)
            .bind(("slug", slug))
            .await?;

        Ok(())
    }

    async fn permission_assign(
        &self,
        role_id: &str,
        permission_id:
        &str,
    ) -> Result<()> {
        match self.db.query(format!(r#"
            RELATE roles:{}->role_permissions->permissions:{};
            "#, role_id, permission_id))
            .await {
            Ok(..) => Ok(()),
            Err(e) => Err(e.into())
        }
    }

    async fn permissions_drop(
        &self,
        role_id: &str,
    ) -> Result<()> {
        self.db.query(r#"
            DELETE type::thing('roles', $role_id)->role_permissions;
            "#)
            .bind(("role_id", role_id))
            .await?;

        Ok(())
    }
}