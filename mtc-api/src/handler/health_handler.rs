use tower_sessions::Session;

use mtc_model::HealthModel;

use crate::handler::Result;
use crate::middleware::auth_middleware::UserSession;
use crate::model::response_model::ApiResponse;

pub async fn health_handler(
    session: Session,
) -> Result<HealthModel> {
    let id = session.auth_id().await?;

    Ok(ApiResponse::Data(HealthModel { id }))
}
