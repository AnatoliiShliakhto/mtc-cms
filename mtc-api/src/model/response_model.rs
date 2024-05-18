use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use crate::model::pagination_model::PaginationModel;

#[derive(Serialize)]
struct ApiData<T: Serialize> {
    data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination: Option<PaginationModel>,
}

pub enum ApiResponse<T: Serialize> {
    Ok,
    Created,
    Data(T),
    DataPage(T, PaginationModel),
    Json(T),
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        match self {
            Self::Ok => StatusCode::OK.into_response(),
            Self::Created => StatusCode::CREATED.into_response(),
            Self::Data(data) => Json(ApiData::<T> { data, pagination: None }).into_response(),
            Self::DataPage(data, pagination) =>
                Json(ApiData::<T> { data, pagination: Some(pagination) }).into_response(),
            Self::Json(data) => Json(data).into_response(),
        }
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
