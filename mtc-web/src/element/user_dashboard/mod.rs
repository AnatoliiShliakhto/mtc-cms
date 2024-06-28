use dioxus::prelude::*;

use mtc_model::auth_model::AuthModelTrait;

use crate::APP_STATE;
use crate::element::user_dashboard::dashboard::Dashboard;
use crate::element::user_dashboard::sign_in::SignIn;

mod sign_in;
mod dashboard;

#[component]
pub fn UserDashboard() -> Element {
    match !APP_STATE.peek().auth.signal().read().is_auth() {
        true => rsx! { SignIn {} },
        false => rsx! { Dashboard {} },
    }
}
