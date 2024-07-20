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

    let mut is_busy = use_signal(|| false);
    let mut is_success = use_signal(|| false);

    let migrate_task = move |event: Event<FormData>| {
        is_busy.set(true);
        if !event.is_login_valid() || !event.is_string_valid("password", 6) {
            APP_STATE
                .peek()
                .modal
                .signal()
                .set(ModalModel::Error(translate!(i18, "errors.fields")));
            is_busy.set(false);
            return;
        }

        spawn(async move {
            let app_state = APP_STATE.read();

            match app_state
                .api
                .migrate(
                    event.get_string("login").to_uppercase(),
                    event.get_string("password"),
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
            autocomplete: "off",
            onsubmit: migrate_task,
            label { class: "w-full gap-2 form-control",
                input { r#type: "text", name: "login",
                    class: "input input-bordered",
                    placeholder: translate!(i18, "messages.login"),
                    minlength: 5,
                    maxlength: 15,
                    required: true,
                }
            }
            label { class: "w-full gap-2 form-control",
                input { r#type: "password", name: "password",
                    class: "input input-bordered",
                    placeholder: translate!(i18, "messages.password"),
                    minlength: 5,
                    maxlength: 15,
                    required: true,
                }
                if !is_busy() {
                    button { class: "mt-2 w-fit self-center btn btn-neutral btn-outline",
                        r#type: "submit",
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
