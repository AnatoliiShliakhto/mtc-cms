use axum::async_trait;

pub mod role_repository;
pub mod user_repository;
pub mod permissions_repository;
pub mod group_repository;
pub mod schema_repository;

#[async_trait]
pub trait RepositoryPaginate<T> {
    async fn get_page(&self, start: usize, limit: usize) -> crate::error::Result<Vec<T>>;
    async fn get_total(&self) -> crate::error::Result<usize>;
}

#[macro_export]
macro_rules! repository_paginate {
    ($repository:ident, $model:ident, $table:literal) => {
        #[async_trait]
        impl RepositoryPaginate<$model> for $repository {
            async fn get_page(&self, start: usize, limit: usize) -> crate::error::Result<Vec<$model>> {
                let mut result = self.db
                .query(r#"SELECT * FROM type::table($table) LIMIT $limit START $start;"#)
                .bind(("table", $table))
                .bind(("start", start - 1))
                .bind(("limit", limit))
                .await?;
                let result: Vec<$model> = result.take(0)?;
                Ok(result)
            }

            async fn get_total(&self) -> crate::error::Result<usize> {
                let mut result = self.db
                .query(r#"SELECT count() FROM type::table($table) GROUP ALL;"#)
                .bind(("table", $table))
                .await?;
                let result: Option<crate::model::pagination_model::CountModel> = result.take(0)?;
                match result {
                    Some(value) => Ok(value.count),
                    _ => Ok(0usize)
                }
            }
        }
    };
}
