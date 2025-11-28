use super::*;
use crate::prelude::{DatabaseError, Error, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;
use surrealdb::{RecordId, Response};
use tracing::error;

pub trait BaseRepository {
    async fn execute_query_with_params_and_pagination<T>(
        &self,
        query: &str,
        query_params: QueryParams,
        pagination: &PageRequest,
    ) -> Result<PageResponse<T>>
    where
        T: Default + Debug + Serialize + DeserializeOwned + Clone + PartialEq;

    async fn execute_query_with_params(
        &self,
        query: &str,
        query_params: QueryParams,
    ) -> Result<Response>;
}

impl BaseRepository for Repository {
    async fn execute_query_with_params_and_pagination<T>(
        &self,
        query: &str,
        query_params: QueryParams,
        page_request: &PageRequest,
    ) -> Result<PageResponse<T>>
    where
        T: Default + Debug + Serialize + DeserializeOwned + Clone + PartialEq,
    {
        info!(
            "Page request: page_index={}, page_size={}",
            page_request.page_index, page_request.page_size
        );
        let page_count_query = r#"
        (SELECT count() FROM ({QUERY}) GROUP ALL)[0].count ?? 0;
    "#
        .replace("{QUERY}", query);

        let page_row_query = format!("{query} {};", page_request.as_sql());

        let pagination_query = r#"
        LET $number_of_rows = {PAGE_COUNT_QUERY}
        LET $page_rows = {PAGE_ROW_QUERY}
        RETURN {
            number_of_pages: type::int(math::ceil(1.0 * $number_of_rows / {PAGE_SIZE})),
            page_size: {PAGE_SIZE},
            page_index: {PAGE_INDEX},
            page_rows: $page_rows
        };
    "#
        .replace("{PAGE_COUNT_QUERY}", &page_count_query)
        .replace("{PAGE_ROW_QUERY}", &page_row_query)
        .replace("{PAGE_SIZE}", &page_request.page_size.to_string())
        .replace("{PAGE_INDEX}", &page_request.page_index.to_string());

        self.execute_query_with_params(&pagination_query, query_params)
            .await?
            .take::<Option<PageResponse<T>>>(2)
            .map_err(|error| Error::SurrealDbError(error))
            .and_then(|page_response_opt| {
                if let Some(page_response) = page_response_opt {
                    info!(
                        "Page response: number_of_pages={}, number_of_page_rows={}",
                        page_response.number_of_pages,
                        page_response.page_rows.len()
                    );
                    Ok(page_response)
                } else {
                    Err(DatabaseError::SomethingWentWrong.into())
                }
            })
    }

    async fn execute_query_with_params(
        &self,
        query: &str,
        query_params: QueryParams,
    ) -> Result<Response> {
        self.database
            .query(query)
            .bind(query_params.ids)
            .bind(query_params.params)
            .await
            .map_err(|error| Error::SurrealDbError(error))
            .and_then(take_successful_response)
    }
}

fn take_successful_response(mut response: Response) -> Result<Response> {
    let errors = response.take_errors();
    if errors.is_empty() {
        Ok(response)
    } else {
        let errors = errors
            .values()
            .map(|error| error.to_string())
            .collect::<String>();
        error!("Error occurred: {}", errors);
        Err(DatabaseError::SomethingWentWrong.into())
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct QueryParams {
    pub params: Value,
    pub ids: HashMap<String, Vec<RecordId>>,
}

impl QueryParams {
    pub fn new(params: Value, ids: HashMap<String, Vec<RecordId>>) -> QueryParams {
        Self { params, ids }
    }

    pub fn from_params(params: Value) -> QueryParams {
        Self {
            params,
            ..Default::default()
        }
    }

    pub fn from_ids(ids: HashMap<String, Vec<RecordId>>) -> QueryParams {
        Self {
            ids,
            ..Default::default()
        }
    }
}
