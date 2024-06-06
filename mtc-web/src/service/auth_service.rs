use dioxus::hooks::UnboundedReceiver;
use dioxus::prelude::{Readable, Writable};
use futures_util::StreamExt;

use mtc_model::auth_model::AuthModel;

use crate::action::auth_action::AuthAction;
use crate::global_signal::{APP, APP_AUTH};
use crate::handler::auth_handler::AuthHandler;
use crate::service::assign_error;

pub async fn auth_service(mut rx: UnboundedReceiver<AuthAction>) {
    let assign_auth_model = |model: AuthModel| {
        if &*APP_AUTH.read_unchecked().id != model.id {
            *APP_AUTH.write_unchecked() = model
        }
    };

    let app_state = &*APP.read_unchecked();

    while let Some(msg) = rx.next().await {
        match msg {
            AuthAction::SignIn(login, password) => {
                app_state.sign_in(login, password)
                    .await
                    .map_err(|e| assign_error(e))
                    .map(|res| assign_auth_model(res))
                    .unwrap_or(())
            }
            AuthAction::Credentials => {
                app_state.credentials()
                    .await
                    .map_err(|e| assign_error(e))
                    .map(|res| assign_auth_model(res))
                    .unwrap_or(())
            }
            AuthAction::SignOut => {
                app_state.sign_out()
                    .await
                    .map_err(|e| assign_error(e))
                    .map(|res| assign_auth_model(res))
                    .unwrap_or(())
            }
        }
    }
}
