use dioxus::prelude::*;

use mtc_model::auth_model::AuthModelTrait;

use crate::APP_STATE;
use crate::element::user_dashboard::dashboard::Dashboard;
use crate::element::user_dashboard::sign_in::SignIn;

mod sign_in;
mod dashboard;

#[component]
pub fn UserDashboard() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();

    match !auth_state.is_auth() {
        true => rsx! { SignIn {} },
        false => rsx! { Dashboard {} },
    }
}
