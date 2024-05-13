use axum::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::api_error::ApiError;

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct Pagination {
    pub total: usize,
    pub per_page: usize,
    pub current_page: usize,
    pub from: usize,
    pub to: usize,
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub next_page_number: usize,
    pub previous_page_number: usize,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct ModelCount {
    pub count: usize,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct DataPagination<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

#[async_trait]
pub trait RepositoryPaginate<T> {
    async fn paginate(&self, start: usize) -> Result<Vec<T>, ApiError>;
    async fn get_total_count(&self) -> Result<ModelCount, ApiError>;
}

#[macro_export]
macro_rules! repository_paginate {
    ($repository:ident, $model:ident, $table:literal) => {
        #[async_trait]
        impl RepositoryPaginate<$model> for $repository {
            async fn paginate(&self, start: usize) -> Result<Vec<$model>, ApiError> {
                let mut result = DB
                .query(r#"SELECT * FROM type::table($table) LIMIT $limit START $start;"#)
                .bind(("table", $table))
                .bind(("limit", CFG.rows_per_page))
                .bind(("start", start))
                .await?;
                let result: Vec<$model> = result.take(0)?;
                Ok(result)
            }

            async fn get_total_count(&self) -> Result<crate::paginator::ModelCount, ApiError> {
                let mut result = DB
                .query(r#"SELECT count() FROM type::table($table) GROUP ALL;"#)
                .bind(("table", $table))
                .await?;
                let result: Option<crate::paginator::ModelCount> = result.take(0)?;
                result.ok_or(ApiError::from(DbError::EntryNotFound))
            }
        }
    };
}

#[derive(Debug, Serialize)]
pub struct ModelPagination<T> {
    pub data: T,
    pub pagination: Pagination,
}

#[async_trait]
pub trait ServicePaginate<T> {
    async fn paginate(&self, current_page: usize) -> Result<ModelPagination<Vec<T>>, ApiError>;
}
#[macro_export]
macro_rules! service_paginate {
    ($service:ident, $model:ident) => {
        #[async_trait]
        impl ServicePaginate<$model> for $service {
            async fn paginate(&self, current_page: usize)
            -> Result<crate::paginator::ModelPagination<Vec<$model>>, ApiError> {
                let current_page = match current_page {
                    n if n > 1 => n,
                    _ => 1,
                };

                let start = (current_page - 1) * CFG.rows_per_page;
                let to = start + CFG.rows_per_page;

                let model_count = self
                    .repository
                    .get_total_count()
                    .await?;

                let mut has_next_page = false;
                if model_count.count > to {
                    has_next_page = true;
                };
                let mut has_previous_page = false;
                if current_page > 1 {
                    has_previous_page = true;
                };


                let pagination = Pagination {
                    total: model_count.count,
                    per_page: CFG.rows_per_page,
                    current_page,
                    from: (start + 1),
                    to,
                    has_previous_page,
                    has_next_page,
                    next_page_number: (current_page + 1),
                    previous_page_number: (current_page - 1),
                };

                let items = self
                    .repository
                    .paginate(start)
                    .await?;

                Ok(ModelPagination {
                    data: items,
                    pagination,
                })
            }
        }
    };
}