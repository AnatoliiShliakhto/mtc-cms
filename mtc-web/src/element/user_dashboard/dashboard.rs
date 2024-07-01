use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::APP_STATE;
use crate::handler::auth_handler::AuthHandler;
use crate::model::modal_model::ModalModel;

#[component]
pub fn Dashboard() -> Element {
    let i18 = use_i18();
    
    let mut is_busy = use_signal(|| false);
    
    let sign_out = move |_| {
        spawn(async move {
            is_busy.set(true);
            let app_state = APP_STATE.read();

            match app_state.api.sign_out().await {
                Ok(auth_model) => { app_state.auth.signal().set(auth_model) }
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message()))
            }
            is_busy.set(false);
        });    
    };

    rsx! {
        p { class: "self-center text-xl",
            { translate!(i18, "messages.welcome") }
        }
        p { class: "m-4",
            { translate!(i18, "messages.logged_in") }
        }
        if !is_busy() {
            button { class: "w-fit self-center btn btn-error btn-outline",
                onclick: sign_out,
                { translate!(i18, "messages.sign_out") }
            }
        } else {
            div { class: "flex w-fit flex-row gap-4 self-center py-3",
                span { class: "loading loading-spinner loading-md" }
                    span { { translate!(i18, "messages.sign_out") } "..." }
            } 
        }
    }
}