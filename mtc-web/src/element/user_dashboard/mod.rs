use dioxus::prelude::*;

use mtc_model::auth_model::AuthModelTrait;

use crate::element::user_dashboard::dashboard::Dashboard;
use crate::element::user_dashboard::sign_in::SignIn;
use crate::global_signal::APP_AUTH;

mod sign_in;
mod dashboard;

#[component]
pub fn UserDashboard() -> Element {
    let auth = &*APP_AUTH.read();

    match !auth.is_auth() {
        true => rsx! { SignIn {} },
        false => rsx! { Dashboard {} },
    }
}
