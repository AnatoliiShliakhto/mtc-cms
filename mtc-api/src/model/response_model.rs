use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use mtc_model::pagination_model::PaginationModel;

use crate::error::Result;

#[derive(Serialize)]
struct ApiData<T: Serialize + Sized> {
    data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination: Option<PaginationModel>,
}

pub enum ApiResponse<T: Serialize + Sized> {
    Ok,
    Data(T),
    DataPage(T, PaginationModel),
}

impl<T> IntoResponse for ApiResponse<T>
    where T: Serialize + Sized {
    fn into_response(self) -> Response {
        match self {
            Self::Ok => StatusCode::OK.into_response(),
            Self::Data(data) => Json(ApiData::<T> { data, pagination: None }).into_response(),
            Self::DataPage(data, pagination) =>
                Json(ApiData::<T> { data, pagination: Some(pagination) }).into_response(),
        }
    }
}

impl<T: Serialize + Sized> From<T> for ApiResponse<T> {
    fn from(data: T) -> Self {
        ApiResponse::Data(data)
    }
}

pub trait HandlerResult<T: Serialize + Sized> {
    fn ok_ok(self) -> Result<ApiResponse<()>>;
    fn ok_model(self) -> Result<ApiResponse<T>>;
    fn ok_page(self, pagination: PaginationModel) -> Result<ApiResponse<T>>;
}

impl<T: Serialize + Sized> HandlerResult<T> for T {
    fn ok_ok(self) -> Result<ApiResponse<()>> {
        Ok(ApiResponse::Ok)
    }

    fn ok_model(self) -> Result<ApiResponse<T>> {
        Ok(ApiResponse::Data(self))
    }

    fn ok_page(self, pagination: PaginationModel) -> Result<ApiResponse<T>> {
        Ok(ApiResponse::DataPage(self, pagination))
    }
}

#[derive(Serialize)]
pub struct ApiErrorResponse {
    message: Option<String>,
    #[serde(rename = "code")]
    status: u16,
}

impl ApiErrorResponse {
    pub fn send(status: u16, message: Option<String>) -> Response {
        ApiErrorResponse { message, status }.into_response()
    }
}

impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self),
        ).into_response()
    }
}
