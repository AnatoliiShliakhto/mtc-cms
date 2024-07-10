use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::APP_STATE;
use crate::component::message_box::{MessageBoxComponent, MessageBoxComponentKind};
use crate::handler::migration_handler::MigrationHandler;
use crate::model::modal_model::ModalModel;
use crate::service::validator_service::ValidatorService;

#[component]
pub fn Migration() -> Element {
    let i18 = use_i18();

    let mut credentials_submit = use_signal(HashMap::<String, FormValue>::new);
    let mut is_busy = use_signal(|| false);
    let mut is_success = use_signal(|| false);

    let is_login_valid = use_memo(move || {
        credentials_submit.is_field_empty("login") | credentials_submit.is_string_valid("login", 5)
    });
    let is_password_valid = use_memo(move || {
        credentials_submit.is_field_empty("password")
            | credentials_submit.is_string_valid("password", 6)
    });

    let migrate_task = move |_| {
        is_busy.set(true);
        if !credentials_submit.is_string_valid("login", 5)
            || !credentials_submit.is_string_valid("password", 6)
        {
            APP_STATE.peek().modal.signal().set(ModalModel::Error(translate!(i18, "errors.fields")));
            is_busy.set(false);
            return;
        }

        spawn(async move {
            let app_state = APP_STATE.read();

            match app_state
                .api
                .migrate(
                    credentials_submit.get_string("login").to_uppercase(),
                    credentials_submit.get_string("password"),
                )
                .await
            {
                Ok(auth_model) => is_success.set(true),
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message())),
            }
            is_busy.set(false);
        });
    };

    if is_success() {
        return rsx! {
            div { class: "m-10",
                MessageBoxComponent { kind: MessageBoxComponentKind::Success(translate!(i18, "messages.migration_success")) }
            }    
        };
    }

    rsx! {
        form { class: "flex grow flex-col items-center gap-3",
            id: "migration-form",
            prevent_default: "onsubmit oninput",
            autocomplete: "off",
            oninput: move |event| credentials_submit.set(event.values()),
            label { class: "w-full gap-2 form-control",
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
            label { class: "w-full gap-2 form-control",
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
                if !is_busy() {
                    button { class: "mt-2 w-fit self-center btn btn-neutral btn-outline",
                        r#type: "button",
                        prevent_default: "onclick",
                        onclick: migrate_task,
                        { translate!(i18, "messages.migration") }
                    }
                } else {
                    div { class: "flex w-fit flex-row gap-4 self-center py-3",
                        span { class: "loading loading-spinner loading-md" }
                        span { { translate!(i18, "messages.migration") } "..." }
                    }
                }
            }
        }
    }
}
