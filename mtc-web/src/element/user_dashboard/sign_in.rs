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

    let sign_in_task = move |_| {
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
            prevent_default: "oninput onsubmit",
            oninput: move |event| credentials_submit.set(event.values()),
            label { class: "form-control w-full gap-2",
                input { r#type: "text", name: "login", class: "input input-bordered",
                    placeholder: translate!(i18, "messages.login"),
                }
                if !credentials_submit.is_field_empty("login") && !credentials_submit.is_string_valid("login", 5) {
                    div { class: "label",
                         span { class: "label-text-alt text-error",
                            { translate!(i18, "validate.login") }
                         }
                    }
                }
            }
            label { class: "form-control w-full gap-2",
                input { r#type: "password", name: "password", class: "input input-bordered",
                    placeholder: translate!(i18, "messages.password")
                }
                if !credentials_submit.is_field_empty("password") && !credentials_submit.is_string_valid("password", 6) {
                    div { class: "label",
                         span { class: "label-text-alt text-error",
                            { translate!(i18, "validate.password") }
                         }
                    }
                }
                if !is_busy() && credentials_submit.is_string_valid("password", 5) && credentials_submit.is_string_valid("password", 6) {
                    button { class: "btn btn-neutral btn-outline w-fit self-center mt-2",
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