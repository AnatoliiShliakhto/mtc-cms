use axum::async_trait;

use mtc_model::api_model::{ApiModel, ApiPostModel};
use mtc_model::pagination_model::CountModel;

use crate::error::db_error::DbError;
use crate::error::Result;
use crate::service::api_service::ApiService;

#[async_trait]
pub trait ApiRepositoryTrait {
    async fn find(&self, table: &str, id: &str) -> Result<ApiModel>;
    async fn find_by_slug(&self, table: &str, slug: &str) -> Result<ApiModel>;
    async fn create(
        &self,
        auth: &str,
        table: &str,
        slug: &str,
        model: ApiPostModel,
    ) -> Result<ApiModel>;
    async fn update(
        &self,
        auth: &str,
        table: &str,
        slug: &str,
        model: ApiPostModel,
    ) -> Result<ApiModel>;
    async fn get_page(&self, table: &str, start: usize, limit: usize) -> Result<Vec<ApiModel>>;
    async fn get_total(&self, table: &str) -> Result<usize>;
}

#[async_trait]
impl ApiRepositoryTrait for ApiService {
    async fn find(&self, table: &str, id: &str) -> Result<ApiModel> {
        self.db
            .query(
                r#"
            SELECT * FROM type::thing($table, $id);
            "#,
            )
            .bind(("table", table))
            .bind(("id", id))
            .await?
            .take::<Option<ApiModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn find_by_slug(&self, table: &str, slug: &str) -> Result<ApiModel> {
        self.db
            .query(
                r#"
            SELECT * FROM type::table($table) WHERE slug=$slug;
            "#,
            )
            .bind(("table", table))
            .bind(("slug", slug))
            .await?
            .take::<Option<ApiModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn create(
        &self,
        auth: &str,
        table: &str,
        slug: &str,
        model: ApiPostModel,
    ) -> Result<ApiModel> {
        self.db
            .query(
                r#"
            CREATE type::table($table) CONTENT {
	            slug: $slug,
	            fields: $fields,
	            created_by: $auth_id,
	            updated_by: $auth_id
            };
            "#,
            )
            .bind(("auth_id", auth))
            .bind(("table", table))
            .bind(("slug", slug))
            .bind(("fields", model.fields))
            .await?
            .take::<Option<ApiModel>>(0)?
            .ok_or(DbError::EntryAlreadyExists.into())
    }

    async fn update(
        &self,
        auth: &str,
        table: &str,
        slug: &str,
        model: ApiPostModel,
    ) -> Result<ApiModel> {
        self.db
            .query(
                r#"
            UPDATE type::table($table) MERGE {
                fields: $fields,
                updated_by: $auth_id
            } WHERE slug=$slug;
            "#,
            )
            .bind(("auth_id", auth))
            .bind(("table", table))
            .bind(("slug", slug))
            .bind(("fields", model.fields))
            .await?
            .take::<Option<ApiModel>>(0)?
            .ok_or(DbError::EntryUpdate.into())
    }

    async fn get_page(&self, table: &str, start: usize, limit: usize) -> Result<Vec<ApiModel>> {
        Ok(self
            .db
            .query(r#"SELECT * FROM type::table($table) LIMIT $limit START $start;"#)
            .bind(("table", table))
            .bind(("start", start - 1))
            .bind(("limit", limit))
            .await?
            .take::<Vec<ApiModel>>(0)?)
    }

    async fn get_total(&self, table: &str) -> Result<usize> {
        match self
            .db
            .query(r#"SELECT count() FROM type::table($table) GROUP ALL;"#)
            .bind(("table", table))
            .await?
            .take::<Option<CountModel>>(0)?
        {
            Some(value) => Ok(value.count),
            _ => Ok(0usize),
        }
    }
}
