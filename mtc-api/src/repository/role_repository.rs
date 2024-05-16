use axum::async_trait;

use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::Result;
use crate::model::role_model::{RoleCreateModel, RoleModel, RoleUpdateModel};
use crate::paginator::RepositoryPaginate;
use crate::provider::config_provider::CFG;
use crate::provider::database_provider::DB;
use crate::repository_paginate;

pub struct RoleRepository;

repository_paginate!(RoleRepository, RoleModel, "roles");

#[async_trait]
pub trait RoleRepositoryTrait {
    async fn find(&self, id: &str) -> Result<RoleModel>;
    async fn find_by_name(&self, name: &str) -> Result<RoleModel>;
    async fn create(&self, model: RoleCreateModel) -> Result<RoleModel>;
    async fn update(&self, id: &str, model: RoleUpdateModel) -> Result<RoleModel>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn permission_assign(&self, role_id: &str, permission_id: &str) -> Result<()>;
    async fn permission_unassign(&self, role_id: &str, permission_id: &str) -> Result<()>;
}

#[async_trait]
impl RoleRepositoryTrait for RoleRepository {
    async fn find(
        &self,
        id: &str,
    ) -> Result<RoleModel> {
        let result: Option<RoleModel> = DB.query(r#"
            SELECT * FROM type::thing('roles', $id);
            "#)
            .bind(("id", id.to_string()))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn find_by_name(
        &self,
        name: &str,
    ) -> Result<RoleModel> {
        let result: Option<RoleModel> = DB.query(r#"
            SELECT * FROM roles WHERE name=$name;
            "#)
            .bind(("name", name.to_string()))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn create(
        &self,
        model: RoleCreateModel,
    ) -> Result<RoleModel> {
        let result: Option<RoleModel> = DB.query(r#"
            CREATE roles CONTENT {
	            name: $name,
	            title: $title
            };
            "#)
            .bind(("name", model.name))
            .bind(("title", model.title))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryAlreadyExists))
        }
    }

    async fn update(
        &self,
        id: &str,
        model: RoleUpdateModel,
    ) -> Result<RoleModel> {
        let result: Option<RoleModel> = DB.query(r#"
            UPDATE type::thing('roles', $id) MERGE {
	            name: $name,
	            title: $title
            } WHERE id;
            "#)
            .bind(("id", id))
            .bind(("name", model.name))
            .bind(("title", model.title))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryUpdate))
        }
    }

    async fn delete(
        &self,
        id: &str,
    ) -> Result<()> {
        match DB.query(r#"
            DELETE type::thing('roles', $id);
            "#)
            .bind(("id", id))
            .await {
            Ok(..) => Ok(()),
            Err(e) => Err(ApiError::from(e))
        }
    }

    async fn permission_assign(&self, role_id: &str, permission_id: &str) -> Result<()> {
        match DB.query(format!(r#"
            RELATE roles:{}->role_permissions->permissions:{};
            "#, role_id, permission_id))
            .await {
            Ok(..) => Ok(()),
            Err(e) => Err(ApiError::from(e))
        }
    }

    async fn permission_unassign(&self, role_id: &str, permission_id: &str) -> Result<()> {
        match DB.query(r#"
            DELETE type::thing('roles', $role_id)->role_permissions WHERE out=type::thing('permissions', $permission_id);
            "#)
            .bind(("role_id", role_id))
            .bind(("permission_id", permission_id))
            .await {
            Ok(..) => Ok(()),
            Err(e) => Err(ApiError::from(e))
        }
    }
}