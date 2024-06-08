use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;

use crate::element::user_dashboard::dashboard::Dashboard;
use crate::element::user_dashboard::sign_in::SignIn;
use crate::global_signal::APP_AUTH;

mod sign_in;
mod dashboard;

#[component]
pub fn UserDashboard() -> Element {
    let i18 = use_i18();
    let auth = &*APP_AUTH.read();

    rsx! {
        div { class: "flex flex-col pt-3",
            p { class: "m-fit text-3xl py-2 self-center",
                { translate!(i18, "messages.dashboard") }
            }
            div { class: "flex flex-col m-auto gap-3 p-5 my-5 self-center min-w-72 md:min-w-96 border input-bordered shadow-md rounded",
                match !auth.is_auth() {
                    true => rsx! { SignIn {} },
                    false => rsx! { Dashboard {} }
                }
            }
        }
    }
}
