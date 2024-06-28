use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::APP_STATE;
use crate::handler::auth_handler::AuthHandler;
use crate::model::modal_model::ModalModel;
use crate::service::validator_service::ValidatorService;

#[component]
pub fn SignIn() -> Element {
    let i18 = use_i18();

    let mut credentials_submit = use_signal(HashMap::<String, FormValue>::new);
    let mut is_busy = use_signal(|| false);

    let is_login_valid = use_memo(move || credentials_submit.is_field_empty("login") | credentials_submit.is_string_valid("login", 5));
    let is_password_valid = use_memo(move || credentials_submit.is_field_empty("password") | credentials_submit.is_string_valid("password", 6));

    let sign_in_task = move |_| {
        if !credentials_submit.is_string_valid("login", 5) || 
            !credentials_submit.is_string_valid("password", 6) { return }
        
        spawn(async move {
            is_busy.set(true);
            let app_state = APP_STATE.read();

            match app_state.api.sign_in(
                credentials_submit.get_string("login"),
                credentials_submit.get_string("password"),
            ).await {
                Ok(auth_model) => { app_state.auth.signal().set(auth_model) }
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message()))
            }
            is_busy.set(false);
        });
    };

    rsx! {
        form { class: "flex flex-col gap-3 grow items-center px-3",
            id: "credentials-form",
            prevent_default: "onsubmit oninput",
            autocomplete: "off",
            oninput: move |event| credentials_submit.set(event.values()),
            label { class: "form-control w-full gap-2",
                input { r#type: "text", name: "login", 
                    class: if is_login_valid() { "input input-bordered" } else { "input input-bordered input-error" },
                    placeholder: translate!(i18, "messages.login"),
                    autofocus: true,
                }
                if !is_login_valid() {
                    div { class: "label",
                         span { class: "label-text-alt text-error",
                            { translate!(i18, "validate.login") }
                         }
                    }
                }
            }
            label { class: "form-control w-full gap-2",
                input { r#type: "password", name: "password", 
                    class: if is_password_valid() { "input input-bordered" } else { "input input-bordered input-error" },
                    placeholder: translate!(i18, "messages.password")
                }
                if !is_password_valid() {
                    div { class: "label",
                         span { class: "label-text-alt text-error",
                            { translate!(i18, "validate.password") }
                         }
                    }
                }
                if !is_busy() && is_login_valid() && is_password_valid() {
                    button { class: "btn btn-neutral btn-outline w-fit self-center mt-2",
                        r#type: "button",
                        prevent_default: "onclick",
                        onclick: sign_in_task,
                        { translate!(i18, "messages.sign_in") }
                    }
                } else if is_busy() {
                    div { class: "flex flex-row gap-4 py-3 w-fit self-center",
                        span { class: "loading loading-spinner loading-md" }
                        span { { translate!(i18, "messages.sign_in") } "..." }
                    }                    
                }
            }
        }
    }
}