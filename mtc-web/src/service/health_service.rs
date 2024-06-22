use dioxus::prelude::*;
use tracing::error;

use crate::APP_STATE;
use crate::handler::auth_handler::AuthHandler;
use crate::handler::health_handler::HealthHandler;
use crate::service::AppService;

pub trait HealthService {
    fn health_check(&self);
    fn get_credentials(&self);
}

impl HealthService for AppService {
    fn health_check(&self) {
        spawn(async move {
            let app_state = APP_STATE.peek();
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

    fn get_credentials(&self) {
        spawn( async move {
            let app_state = APP_STATE.peek();
            
            match app_state.api.get_credentials().await {
                Ok(auth_model) => app_state.auth.signal().set(auth_model),
                Err(e) => error!("Get credentials: {}", e.message()),
            }
        });
    }
}