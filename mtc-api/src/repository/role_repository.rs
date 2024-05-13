use axum::async_trait;

use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::model::role_model::{RoleCreateModel, RoleModel, RoleUpdateModel};
use crate::paginator::RepositoryPaginate;
use crate::provider::config_provider::CFG;
use crate::provider::database_provider::DB;
use crate::repository_paginate;

pub struct RoleRepository;

repository_paginate!(RoleRepository, RoleModel, "roles");

#[async_trait]
pub trait RoleRepositoryTrait {
    async fn find(&self, id: &str) -> Result<RoleModel, ApiError>;
    async fn create(&self, role_create_model: RoleCreateModel) -> Result<RoleModel, ApiError>;
    async fn update(&self, id: &str, role_update_model: RoleUpdateModel) -> Result<RoleModel, ApiError>;
    async fn delete(&self, id: &str) -> Result<(), ApiError>;
}

#[async_trait]
impl RoleRepositoryTrait for RoleRepository {
    async fn find(&self, id: &str) -> Result<RoleModel, ApiError> {
        let result: Option<RoleModel> = DB.query(r#"
            SELECT * FROM type::thing($table, $id);
            "#).bind(("table", "roles"))
            .bind(("id", id.to_string()))
            .await?.take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn create(&self, role_create_model: RoleCreateModel) -> Result<RoleModel, ApiError> {
        let result: Option<RoleModel> = DB.query(r#"
            CREATE type::table($table) CONTENT {
	            name: $name,
	            title: $title
            };
            "#).bind(("table", "roles"))
            .bind(("name", role_create_model.name))
            .bind(("title", role_create_model.title))
            .await?.take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryAlreadyExists))
        }
    }

    async fn update(&self, id: &str, role_update_model: RoleUpdateModel) -> Result<RoleModel, ApiError> {
        let result: Option<RoleModel> = DB.query(r#"
            UPDATE type::thing($table, $id) MERGE {
	            name: $name,
	            title: $title
            } WHERE id;
            "#).bind(("table", "roles"))
            .bind(("id", id))
            .bind(("name", role_update_model.name))
            .bind(("title", role_update_model.title))
            .await?.take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryUpdate))
        }
    }

    async fn delete(&self, id: &str) -> Result<(), ApiError> {
        DB.query(r#"
            DELETE type::thing($table, $id);
            "#).bind(("table", "roles"))
            .bind(("id", id))
            .await.map_err(|_| ApiError::from(DbError::EntryDelete))?;

        Ok(())
    }
}