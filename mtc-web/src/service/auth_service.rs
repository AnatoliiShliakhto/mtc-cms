use dioxus::prelude::*;
use tracing::error;

use crate::APP_STATE;
use crate::handler::auth_handler::AuthHandler;
use crate::service::AppService;

pub trait AuthService {
    fn sign_in(&self, login: Signal<String>, password: Signal<String>, error: Option<Signal<String>>);
    fn sign_out(&self);
    fn get_credentials(&self);
}

impl AuthService for AppService {
    fn sign_in(
        &self,
        login: Signal<String>,
        password: Signal<String>,
        error: Option<Signal<String>>,
    ) {
        spawn(async move {
            let app_state = APP_STATE.read();

            match app_state.api.sign_in(login(), password()).await {
                Ok(auth_model) => app_state.auth.signal().set(auth_model),
                Err(e) => {
                    match error {
                        Some(mut error) => error.set(e.message()),
                        None => error!("SignIn: {}", e.message()),
                    }
                }
            }
        });
    }

    fn sign_out(&self) {
        spawn(async move {
            let app_state = APP_STATE.read();

            match app_state.api.sign_out().await {
                Ok(auth_model) => app_state.auth.signal().set(auth_model),
                Err(e) => error!("SignOut: {}", e.message())
            }
        });
    }

    fn get_credentials(&self) {
        spawn(async move {
            let app_state = APP_STATE.read();

            match app_state.api.get_credentials().await {
                Ok(auth_model) => app_state.auth.signal().set(auth_model),
                Err(e) => error!("Get credentials: {}", e.message()),
            }
        });
    }
}