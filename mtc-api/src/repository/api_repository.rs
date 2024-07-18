use axum::async_trait;

use mtc_model::api_model::{ApiListItemModel, ApiModel, ApiPostModel};
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
    async fn delete(&self, table: &str, slug: &str) -> Result<()>;
    async fn get_page(
        &self,
        table: &str,
        start: usize,
        limit: usize,
        is_admin: bool,
    ) -> Result<Vec<ApiModel>>;
    async fn get_total(&self, table: &str, is_admin: bool) -> Result<usize>;
    async fn get_all_items(&self, table: &str) -> Result<Vec<ApiListItemModel>>;
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
	                title: $title,
	                published: $published,
	                fields: $fields,
	                created_by: $auth_id,
	                updated_by: $auth_id
                };
            "#,
            )
            .bind(("auth_id", auth))
            .bind(("table", table))
            .bind(("slug", slug))
            .bind(("title", model.title))
            .bind(("published", model.published))
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
                    title: $title,
                    published: $published,
                    fields: $fields,
                    updated_by: $auth_id
                } WHERE slug=$slug;
            "#,
            )
            .bind(("auth_id", auth))
            .bind(("table", table))
            .bind(("slug", slug))
            .bind(("title", model.title))
            .bind(("published", model.published))
            .bind(("fields", model.fields))
            .await?
            .take::<Option<ApiModel>>(0)?
            .ok_or(DbError::EntryUpdate.into())
    }

    async fn delete(&self, table: &str, slug: &str) -> Result<()> {
        self.db
            .query(
                r#"
                DELETE FROM type::table($table) WHERE slug=$slug;
            "#,
            )
            .bind(("table", table))
            .bind(("slug", slug))
            .await?;

        Ok(())
    }

    async fn get_page(
        &self,
        table: &str,
        start: usize,
        limit: usize,
        is_admin: bool,
    ) -> Result<Vec<ApiModel>> {
        Ok(self
            .db
            .query(match is_admin {
                true => r#"SELECT * FROM type::table($table) LIMIT $limit START $start;"#,
                false => r#"SELECT * FROM type::table($table) WHERE published = true LIMIT $limit START $start;"#,
            })
            .bind(("table", table))
            .bind(("start", start - 1))
            .bind(("limit", limit))
            .await?
            .take::<Vec<ApiModel>>(0)?)
    }

    async fn get_total(&self, table: &str, is_admin: bool) -> Result<usize> {
        match self
            .db
            .query(match is_admin {
                true => r#"SELECT count() FROM type::table($table) GROUP ALL;"#,
                false => {
                    r#"SELECT count() FROM type::table($table) WHERE published = true GROUP ALL;"#
                }
            })
            .bind(("table", table))
            .await?
            .take::<Option<CountModel>>(0)?
        {
            Some(value) => Ok(value.count),
            _ => Ok(0usize),
        }
    }

    async fn get_all_items(&self, table: &str) -> Result<Vec<ApiListItemModel>> {
        Ok(self
            .db
            .query(r#"SELECT slug, title, published FROM type::table($table);"#)
            .bind(("table", table))
            .await?
            .take::<Vec<ApiListItemModel>>(0)?)
    }
}
