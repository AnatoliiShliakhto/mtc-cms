use std::sync::Arc;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use tower_sessions::Session;

use crate::error::api_error::ApiError;
use crate::state::AppState;

pub async fn auth_handler(
    state: State<Arc<AppState>>,
    session: Session,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, ApiError> {


    session.insert("role", "anonymous").await.unwrap();
    Ok(next.run(req).await)
}