use dioxus::prelude::*;
use tracing::error;

use crate::APP_STATE;
use crate::handler::health_handler::HealthHandler;
use crate::service::AppService;
use crate::service::auth_service::AuthService;

pub trait HealthService {
    fn health_check(&self);
}

impl HealthService for AppService {
    fn health_check(&self) {
        spawn(async move {
            let app_state = APP_STATE.read();
            let auth_state = app_state.auth.read();

            match app_state.api.health_check().await {
                Ok(health_model) => {
                    if auth_state.id.ne(&health_model.id) {
                        app_state.service.get_credentials()
                    }
                }
                Err(e) => error!("API health: {}", e.message())
            }
        });
    }
}