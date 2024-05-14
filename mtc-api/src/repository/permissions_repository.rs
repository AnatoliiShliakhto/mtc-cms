use axum::async_trait;

use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::model::StringListModel;
use crate::model::permission_model::{PermissionCreateModel, PermissionModel};
use crate::provider::database_provider::DB;

pub struct PermissionsRepository;

#[async_trait]
pub trait PermissionsRepositoryTrait {
    async fn all(&self) -> Result<Vec<PermissionModel>, ApiError>;
    async fn find_by_role(&self, role: &str) -> Result<Vec<String>, ApiError>;
    async fn create(&self, permission_create_model: PermissionCreateModel) -> Result<PermissionModel, ApiError>;
    async fn delete(&self, id: &str) -> Result<(), ApiError>;
}

#[async_trait]
impl PermissionsRepositoryTrait for PermissionsRepository {
    async fn all(&self) -> Result<Vec<PermissionModel>, ApiError> {
        let result: Vec<PermissionModel> = DB.query(r#"
            SELECT * FROM type::table($table);
            "#).bind(("table", "permissions"))
            .await?.take(0)?;

        Ok(result)
    }

    async fn find_by_role(&self, role_id: &str) -> Result<Vec<String>, ApiError> {
        let result: Option<StringListModel> = DB.query(r#"
            SELECT array::distinct(->role_permissions->permissions.name) as items FROM type::thing($table, $id);
            "#).bind(("table", "roles"))
            .bind(("id", role_id.to_string()))
            .await?.take(0)?;

        match result {
            Some(value) => Ok(value.items),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn create(&self, permission_create_model: PermissionCreateModel) -> Result<PermissionModel, ApiError> {
        let result: Option<PermissionModel> = DB.query(r#"
            CREATE type::table($table) CONTENT {
	            name: $name,
	            title: $title
            };
            "#).bind(("table", "permissions"))
            .bind(("name", permission_create_model.name))
            .await?.take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryAlreadyExists))
        }
    }

    async fn delete(&self, id: &str) -> Result<(), ApiError> {
        DB.query(r#"
            BEGIN TRANSACTION;
            DELETE type::thing($table, $id);
            DELETE FROM type::table($rel_table) WHERE IN = type::thing($table, $id) OR OUT = type::thing($table, $id);
            COMMIT TRANSACTION;
            "#).bind(("table", "permissions"))
            .bind(("rel_table", "role_permissions"))
            .bind(("id", id))
            .await.map_err(|_| ApiError::from(DbError::EntryDelete))?;

        Ok(())
    }
}