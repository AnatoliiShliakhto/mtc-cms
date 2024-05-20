use axum::async_trait;

use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::Result;
use crate::model::schema_model::SchemaModel;
use crate::repository::RepositoryPaginate;
use crate::repository_paginate;
use crate::service::schema_service::SchemaService;

repository_paginate!(SchemaService, SchemaModel, "schemas");

#[async_trait]
pub trait SchemaRepositoryTrait {
    async fn find(&self, id: &str) -> Result<SchemaModel>;
    async fn find_by_name(&self, name: &str) -> Result<SchemaModel>;
    async fn can_create(&self, name: &str) -> Result<bool>;
    async fn can_delete(&self, name: &str) -> Result<bool>;
}

#[async_trait]
impl SchemaRepositoryTrait for SchemaService {
    async fn find(
        &self,
        id: &str,
    ) -> Result<SchemaModel> {
        let result: Option<SchemaModel> = self.db.query(r#"
            SELECT * FROM type::thing('schemas', $id);
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
    ) -> Result<SchemaModel> {
        let result: Option<SchemaModel> = self.db.query(r#"
            SELECT * FROM schemas WHERE name=$name;
            "#)
            .bind(("name", name.to_string()))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn can_create(
        &self,
        name: &str,
    ) -> Result<bool> {
        let result: Option<String> = self.db.query(r#"
            SELECT name FROM schemas WHERE name=$name;
            "#)
            .bind(("name", name))
            .await?
            .take(0)?;

        match result {
            Some(..) => Ok(false),
            None => Ok(true)
        }
    }

    async fn can_delete(
        &self,
        name: &str,
    ) -> Result<bool> {
        let result: Option<bool> = self.db.query(r#"
            SELECT is_system FROM schemas WHERE name=$name;
            "#)
            .bind(("name", name))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(!value),
            None => Ok(false)
        }
    }
}
