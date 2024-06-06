use serde::Deserialize;

use mtc_model::pagination_model::PaginationModel;

#[derive(Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationModel>,
}

#[derive(Deserialize)]
pub struct ApiErrorResponse {
    pub message: Option<String>,
    pub code: u16,
}