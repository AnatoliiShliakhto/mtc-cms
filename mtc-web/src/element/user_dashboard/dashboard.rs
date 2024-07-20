use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::handler::auth_handler::AuthHandler;
use crate::model::modal_model::ModalModel;
use crate::service::validator_service::ValidatorService;
use crate::APP_STATE;

#[component]
pub fn Dashboard() -> Element {
    let i18 = use_i18();

    let mut is_busy = use_signal(|| false);

    let sign_out = move |event: Event<MouseData>| {
        spawn(async move {
            is_busy.set(true);
            let app_state = APP_STATE.read();

            match app_state.api.sign_out().await {
                Ok(auth_model) => app_state.auth.signal().set(auth_model),
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message())),
            }
            is_busy.set(false);
        });
    };

    let password_submit = move |event: Event<FormData>| {
        is_busy.set(true);

        if event.get_string("confirm-password") != event.get_string("new-password")
        {
            APP_STATE
                .peek()
                .modal
                .signal()
                .set(ModalModel::Error(translate!(i18, "errors.passwords_match")));
            is_busy.set(false);
            return;
        };
        
        if !event.is_string_valid("old-password", 6) | !event.is_string_valid("new-password", 6)
        {
            APP_STATE
                .peek()
                .modal
                .signal()
                .set(ModalModel::Error(translate!(i18, "errors.fields")));
            is_busy.set(false);
            return;
        };

        spawn(async move {
            let app_state = APP_STATE.read();
            match app_state
                .api
                .change_password(
                    &event.get_string("old-password"),
                    &event.get_string("new-password"),
                )
                .await
            {
                Ok(_) => app_state.modal.signal().set(ModalModel::Success(translate!(
                    i18,
                    "messages.password_change_success"
                ))),
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message())),
            }
            is_busy.set(false);
        });
    };

    rsx! {
        p { class: "self-center text-xl",
            { translate!(i18, "messages.welcome") }
        }
        p { class: "m-4 self-center",
            { translate!(i18, "messages.logged_in") }
        }
        form { class: "flex grow flex-col items-center gap-3",
            id: "password-form",
            autocomplete: "off",
            onsubmit: password_submit,
            label { class: "w-full gap-2 form-control",
                input { r#type: "password", name: "old-password",
                    class: "input input-bordered",
                    placeholder: translate!(i18, "messages.password_old"),
                    minlength: 6,
                    maxlength: 20,
                    required: true,
                }
            }

            label { class: "w-full gap-2 form-control",
                input { r#type: "password", name: "new-password",
                    class: "input input-bordered",
                    placeholder: translate!(i18, "messages.password_new"),
                    minlength: 6,
                    maxlength: 20,
                    required: true,
                }
            }

            label { class: "w-full gap-2 form-control",
                input { r#type: "password", name: "confirm-password",
                    class: "input input-bordered",
                    placeholder: translate!(i18, "messages.password_confirm"),
                    minlength: 6,
                    maxlength: 20,
                    required: true,
                }
            }
        }
        div { class: "inline-flex justify-between gap-5 pt-5",
            if !is_busy() {
                button { class: "w-fit btn btn-outline",
                    r#type: "submit",
                    form: "password-form",
                    Icon {
                        width: 22,
                        height: 22,
                        icon: dioxus_free_icons::icons::md_action_icons::MdLockOutline
                    }
                    { translate!(i18, "messages.password_change") }
                }
                button { class: "w-fit btn btn-error btn-outline",
                    onclick: sign_out,
                    Icon {
                        width: 22,
                        height: 22,
                        icon: dioxus_free_icons::icons::md_action_icons::MdLogout
                    }
                    { translate!(i18, "messages.sign_out") }
                }
            } else {
                div { class: "flex w-full justify-center flex-row gap-4 py-3",
                    span { class: "loading loading-spinner loading-md" }
                    span { { translate!(i18, "messages.processing") } "..." }
                }
            }
        }
    }
}
