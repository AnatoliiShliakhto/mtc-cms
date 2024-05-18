use axum::async_trait;

use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::Result;
use crate::model::group_model::{GroupCreateModel, GroupModel, GroupUpdateModel};
use crate::repository::RepositoryPaginate;
use crate::repository_paginate;
use crate::service::group_service::GroupService;

repository_paginate!(GroupService, GroupModel, "groups");

#[async_trait]
pub trait GroupRepositoryTrait {
    async fn find(&self, id: &str) -> Result<GroupModel>;
    async fn create(&self, model: GroupCreateModel) -> Result<GroupModel>;
    async fn update(&self, id: &str, model: GroupUpdateModel) -> Result<GroupModel>;
    async fn delete(&self, id: &str) -> Result<()>;
}

#[async_trait]
impl GroupRepositoryTrait for GroupService {
    async fn find(
        &self,
        id: &str,
    ) -> Result<GroupModel> {
        let result: Option<GroupModel> = self.db.query(r#"
            SELECT * FROM type::thing('groups', $id);
            "#)
            .bind(("id", id.to_string()))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn create(
        &self,
        model: GroupCreateModel,
    ) -> Result<GroupModel> {
        let result: Option<GroupModel> = self.db.query(r#"
            CREATE groups CONTENT {
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
        model: GroupUpdateModel,
    ) -> Result<GroupModel> {
        let result: Option<GroupModel> = self.db.query(r#"
            UPDATE type::thing('groups', $id) MERGE {
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
        match self.db.query(r#"
            DELETE type::thing('groups', $id);
            "#)
            .bind(("id", id))
            .bind(("rel_table", "user_groups"))
            .await {
            Ok(..) => Ok(()),
            Err(e) => Err(ApiError::from(e))
        }
    }
}