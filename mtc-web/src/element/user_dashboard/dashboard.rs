use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::APP_STATE;
use crate::service::auth_service::AuthService;

#[component]
pub fn Dashboard() -> Element {
    let app_state = APP_STATE.peek();
    let i18 = use_i18();

    rsx! {
        p { class: "text-xl self-center",
            { translate!(i18, "messages.welcome") }
        }
        p { class: "m-4",
            { translate!(i18, "messages.logged_in") }
        }
        button { class: "btn btn-error btn-outline w-fit self-center",
            onclick: move |_| app_state.service.sign_out(),
            { translate!(i18, "messages.sign_out") }
        }
    }
}