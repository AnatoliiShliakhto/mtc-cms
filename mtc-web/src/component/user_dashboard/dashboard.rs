use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::action::auth_action::AuthAction;

#[component]
pub fn Dashboard() -> Element {
    let i18 = use_i18();

    rsx! {
        div { class: "flex flex-col m-auto gap-6 p-10 rounded-lg my-10 shadow-lg hover:shadow-xl self-center min-w-96",
            p { class: "text-xl self-center",
                { translate!(i18, "messages.welcome") }
            }
            p { class: "m-4",
            { translate!(i18, "messages.logged_in") }
            }
            button { class: "btn btn-primary w-fit self-center",
                onclick: move |_| {
                    spawn(async move {
                        use_coroutine_handle::<AuthAction>().send(AuthAction::SignOut);
                    });
                },
                { translate!(i18, "messages.sign_out") }
            }
        }
    }
}