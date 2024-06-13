use dioxus::prelude::{Readable, UnboundedReceiver, Writable};
use futures_util::StreamExt;

use crate::action::health_action::HealthAction;
use crate::global_signal::{APP, APP_AUTH};
use crate::handler::auth_handler::AuthHandler;
use crate::handler::health_handler::HealthHandler;
use crate::service::assign_error;

pub async fn health_service(mut rx: UnboundedReceiver<HealthAction>) {
    let app_state = &*APP.read_unchecked();

    while let Some(msg) = rx.next().await {
        match msg {
            HealthAction::Check => {
                match app_state.health_check().await {
                    Ok(health_model) => {
                        if &*APP_AUTH.read_unchecked().id != health_model.id {
                            match app_state.credentials().await {
                                Ok(model) => *APP_AUTH.write_unchecked() = model,
                                Err(e) => assign_error(e)
                            }
                        }
                    }
                    Err(e) => assign_error(e)
                }
            }
        }
    }
}

