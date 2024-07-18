use std::sync::Arc;

use axum::extract::State;
use tower_sessions::cookie::time::Duration;
use tower_sessions::{Expiry, Session};

use mtc_model::HealthModel;

use crate::handler::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::response_model::ApiResponse;
use crate::state::AppState;

pub async fn health_handler(state: State<Arc<AppState>>, session: Session) -> Result<HealthModel> {
    let id = session.auth_id().await?;

    session.set_expiry(Some(Expiry::OnInactivity(Duration::minutes(
        state.cfg.session_expiration,
    ))));

    Ok(ApiResponse::Data(HealthModel { id }))
}
