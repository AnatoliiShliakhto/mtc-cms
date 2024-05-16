use axum::async_trait;

use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::Result;
use crate::model::single_type_model::{SingleTypeCreateModel, SingleTypeModel};
use crate::paginator::RepositoryPaginate;
use crate::provider::config_provider::CFG;
use crate::provider::database_provider::DB;
use crate::repository_paginate;

pub struct SingleTypeRepository;

repository_paginate!(SingleTypeRepository, SingleTypeModel, "single_types");

#[async_trait]
pub trait SingleTypeRepositoryTrait {
    async fn find(&self, id: &str) -> Result<SingleTypeModel>;
    async fn find_by_api(&self, api: &str) -> Result<SingleTypeModel>;
    async fn create(&self, model: SingleTypeCreateModel) -> Result<SingleTypeModel>;
}

#[async_trait]
impl SingleTypeRepositoryTrait for SingleTypeRepository {
    async fn find(
        &self,
        id: &str,
    ) -> Result<SingleTypeModel> {
        let result: Option<SingleTypeModel> = DB.query(r#"
            SELECT * FROM type::thing('single_types', $id);
            "#)
            .bind(("id", id.to_string()))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn find_by_api(
        &self,
        api: &str,
    ) -> Result<SingleTypeModel> {
        let result: Option<SingleTypeModel> = DB.query(r#"
            SELECT * FROM single_types WHERE api=$api;
            "#)
            .bind(("name", api.to_string()))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn create(&self, model: SingleTypeCreateModel) -> Result<SingleTypeModel> {
        let result: Option<SingleTypeModel> = DB.query(r#"
            BEGIN TRANSACTION;

            CREATE tables CONTENT {
                name: $api,
                is_core: false
            };

            CREATE single_types CONTENT {
	            api: $api,
            };

            CREATE permissions CONTENT {
                id: $permission_read_id,
                name: $permission_read_name
            };

            CREATE permissions CONTENT {
                id: $permission_write_id,
                name: $permission_write_name
            };

            CREATE permissions CONTENT {
                id: $permission_delete_id,
                name: $permission_delete_name
            };

            COMMIT TRANSACTION;
            "#)
            .bind(("api", &model.api))
            .bind(("permission_read_id", format!("{}_read", &model.api)))
            .bind(("permission_read_name", format!("{}::read", &model.api)))
            .bind(("permission_write_id", format!("{}_write", &model.api)))
            .bind(("permission_write_name", format!("{}::write", &model.api)))
            .bind(("permission_delete_id", format!("{}_delete", &model.api)))
            .bind(("permission_delete_name", format!("{}::delete", &model.api)))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryAlreadyExists))
        }
    }
}