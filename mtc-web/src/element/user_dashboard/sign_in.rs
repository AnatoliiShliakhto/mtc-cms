use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::handler::auth_handler::AuthHandler;
use crate::model::modal_model::ModalModel;
use crate::service::validator_service::ValidatorService;
use crate::APP_STATE;
use crate::router::Route::HomePage;

#[component]
pub fn SignIn() -> Element {
    let i18 = use_i18();

    let mut is_busy = use_signal(|| false);

    let sign_in_task = move |event: Event<FormData>| {
        is_busy.set(true);

        if !event.is_login_valid() | !event.is_string_valid("password", 6) {
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
                .sign_in(
                    event.get_string("login").to_uppercase(),
                    event.get_string("password"),
                )
                .await
            {
                Ok(auth_model) => {
                    app_state.auth.signal().set(auth_model);
                    
                    navigator().push(HomePage {});
                },
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message())),
            }
            is_busy.set(false);
        });
    };

    rsx! {
        form { class: "flex grow flex-col items-center gap-3",
            autocomplete: "off",
            onsubmit: sign_in_task,
            label { class: "w-full gap-2 form-control",
                input { class: "input input-bordered",
                    r#type: "text",
                    name: "login",
                    placeholder: translate!(i18, "messages.login"),
                    minlength: 5,
                    maxlength: 15,
                    required: true,
                }
            }
            label { class: "w-full gap-2 form-control",
                input { class: "input input-bordered",
                    r#type: "password",
                    name: "password",
                    placeholder: translate!(i18, "messages.password"),
                    minlength: 6,
                    maxlength: 20,
                    required: true,
                }
            }

            if !is_busy() {
                button { class: "mt-2 w-fit self-center btn btn-neutral btn-outline",
                    r#type: "submit",
                    Icon {
                        width: 22,
                        height: 22,
                        icon: dioxus_free_icons::icons::md_action_icons::MdLogin
                    }
                    { translate!(i18, "messages.sign_in") }
                }
            } else {
                div { class: "flex w-fit flex-row gap-4 self-center py-3",
                    span { class: "loading loading-spinner loading-md" }
                    span { { translate!(i18, "messages.sign_in") } "..." }
                }
            }
        }
    }
}
