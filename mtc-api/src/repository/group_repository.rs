use axum::async_trait;

use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::model::group_model::{GroupCreateModel, GroupModel, GroupUpdateModel};
use crate::paginator::RepositoryPaginate;
use crate::provider::config_provider::CFG;
use crate::provider::database_provider::DB;
use crate::repository_paginate;

pub struct GroupRepository;

repository_paginate!(GroupRepository, GroupModel, "groups");

#[async_trait]
pub trait GroupRepositoryTrait {
    async fn find(&self, id: &str) -> Result<GroupModel, ApiError>;
    async fn create(&self, group_create_model: GroupCreateModel) -> Result<GroupModel, ApiError>;
    async fn update(&self, id: &str, group_update_model: GroupUpdateModel) -> Result<GroupModel, ApiError>;
    async fn delete(&self, id: &str) -> Result<(), ApiError>;
}

#[async_trait]
impl GroupRepositoryTrait for GroupRepository {
    async fn find(&self, id: &str) -> Result<GroupModel, ApiError> {
        let result: Option<GroupModel> = DB.query(r#"
            SELECT * FROM type::thing($table, $id);
            "#).bind(("table", "groups"))
            .bind(("id", id.to_string()))
            .await?.take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn create(&self, group_create_model: GroupCreateModel) -> Result<GroupModel, ApiError> {
        let result: Option<GroupModel> = DB.query(r#"
            CREATE type::table($table) CONTENT {
	            name: $name,
	            title: $title
            };
            "#).bind(("table", "groups"))
            .bind(("name", group_create_model.name))
            .bind(("title", group_create_model.title))
            .await?.take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryAlreadyExists))
        }
    }

    async fn update(&self, id: &str, group_update_model: GroupUpdateModel) -> Result<GroupModel, ApiError> {
        let result: Option<GroupModel> = DB.query(r#"
            UPDATE type::thing($table, $id) MERGE {
	            name: $name,
	            title: $title
            } WHERE id;
            "#).bind(("table", "groups"))
            .bind(("id", id))
            .bind(("name", group_update_model.name))
            .bind(("title", group_update_model.title))
            .await?.take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryUpdate))
        }
    }

    async fn delete(&self, id: &str) -> Result<(), ApiError> {
        DB.query(r#"
            BEGIN TRANSACTION;
            DELETE type::thing($table, $id);
            DELETE FROM type::table($rel_table) WHERE IN = type::thing($table, $id) OR OUT = type::thing($table, $id);
            COMMIT TRANSACTION;
            "#).bind(("table", "groups"))
            .bind(("id", id))
            .bind(("rel_table", "user_groups"))
            .await.map_err(|_| ApiError::from(DbError::EntryDelete))?;

        Ok(())
    }
}