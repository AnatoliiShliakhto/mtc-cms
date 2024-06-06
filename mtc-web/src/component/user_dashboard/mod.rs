use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;

use crate::component::user_dashboard::dashboard::Dashboard;
use crate::component::user_dashboard::sign_in::SignIn;
use crate::global_signal::APP_AUTH;

mod sign_in;
mod dashboard;

#[component]
pub fn UserDashboard() -> Element {
    let i18 = use_i18();
    let auth = &*APP_AUTH.read();

    rsx! {
        div { class: "flex flex-col pt-6",
            p { class: "m-fit text-3xl py-2 self-center border-b-2",
                { translate!(i18, "messages.dashboard") }
            }
            match !auth.is_auth() {
                true => rsx! { SignIn {} },
                false => rsx! { Dashboard {} }
            }
        }
    }
}
