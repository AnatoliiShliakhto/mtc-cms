#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::action::auth_action::AuthAction;

#[component]
pub fn Dashboard() -> Element {
    let i18 = use_i18();

    rsx! {
        p { class: "text-xl self-center",
            { translate!(i18, "messages.welcome") }
        }
        p { class: "m-4",
            { translate!(i18, "messages.logged_in") }
        }
        button { class: "btn btn-error btn-outline w-fit self-center",
            onclick: move |_| {
                spawn(async move {
                    use_coroutine_handle::<AuthAction>().send(AuthAction::SignOut);
                });
            },
            { translate!(i18, "messages.sign_out") }
        }
    }
}