use axum::extract::FromRequest;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use crate::error::api_error::ApiError;

#[derive(Deserialize)]
pub struct PageRequest {
    pub page: Option<usize>,
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct ApiJson<T>(pub T);

impl<T> IntoResponse for ApiJson<T>
    where
        axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}