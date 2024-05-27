use axum::async_trait;

use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::Result;
use crate::model::api_model::{ApiModel, ApiPostModel};
use crate::service::api_service::ApiService;

#[async_trait]
pub trait ApiRepositoryTrait {
    async fn find(&self, table: &str, id: &str) -> Result<ApiModel>;
    async fn find_by_slug(&self, table: &str, slug: &str) -> Result<ApiModel>;
    async fn create(&self, table: &str, slug: &str, model: ApiPostModel) -> Result<ApiModel>;
    async fn update(&self, table: &str, slug: &str, model: ApiPostModel) -> Result<ApiModel>;
}

#[async_trait]
impl ApiRepositoryTrait for ApiService {
    async fn find(
        &self,
        table: &str,
        id: &str) -> Result<ApiModel> {
        let result: Option<ApiModel> = self.db.query(r#"
            SELECT * FROM type::thing($table, $id);
            "#)
            .bind(("table", table))
            .bind(("id", id))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn find_by_slug(
        &self,
        table: &str,
        slug: &str) -> Result<ApiModel> {
        let result: Option<ApiModel> = self.db.query(r#"
            SELECT * FROM type::table($table) WHERE slug=$slug;
            "#)
            .bind(("table", table))
            .bind(("slug", slug))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn create(
        &self,
        table: &str,
        slug: &str,
        model: ApiPostModel) -> Result<ApiModel> {
        let result: Option<ApiModel> = self.db.query(r#"
            CREATE type::table($table) CONTENT {
	            slug: $slug,
	            fields: $fields
            };
            "#)
            .bind(("table", table))
            .bind(("slug", slug))
            .bind(("fields", model.fields))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryAlreadyExists))
        }
    }

    async fn update(
        &self,
        table: &str,
        slug: &str,
        model: ApiPostModel,
    ) -> Result<ApiModel> {
        let result: Option<ApiModel> = self.db.query(r#"
            UPDATE type::table($table) MERGE {
                fields: $fields
            } WHERE slug=$slug;
            "#)
            .bind(("table", table))
            .bind(("slug", slug))
            .bind(("fields", model.fields))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryUpdate))
        }
    }
}