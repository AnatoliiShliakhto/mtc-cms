use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::APP_STATE;
use crate::component::message_box::*;
use crate::service::auth_service::AuthService;

#[component]
pub fn SignIn() -> Element {
    let app_state = APP_STATE.peek();
    let i18 = use_i18();

    let login = use_signal(|| String::new());
    let validate_login = use_memo(move || login().len().ge(&5));

    let password = use_signal(|| String::new());
    let validate_password = use_memo(move || password().len().ge(&6));

    let mut in_progress = use_signal(|| false);

    let mut error = use_signal(|| String::new());

    let mut input_update = move |event: Event<FormData>, mut field: Signal<String>| {
        field.set(event.value());
        error.set("".to_string());
        in_progress.set(false);
    };

    let drop_error = move |_| {
        error.set(String::new());
        in_progress.set(false);
    };

    rsx! {
        div { class: if !validate_login() && !login().is_empty() { "tooltip tooltip-open tooltip-top pt-1 mt-8" },
            "data-tip": translate!(i18, "errors.login_validation"),
            label { class: "input input-bordered flex items-center gap-2",
                svg {
                    "fill": "currentColor",
                    "viewBox": "0 0 16 16",
                    "xmlns": "http://www.w3.org/2000/svg",
                    class: "w-4 h-4 opacity-70",
                    path { "d": "M8 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6ZM12.735 14c.618 0 1.093-.561.872-1.139a6.002 6.002 0 0 0-11.215 0c-.22.578.254 1.139.872 1.139h9.47Z" }
                }
                input { r#type: "text", name: "login", value: login, class: "grow",
                    placeholder: translate!(i18, "messages.login"),
                    oninput: move |event| input_update(event, login)
                }
            }
        }

        div { class: if !validate_password() && !password().is_empty() { "tooltip tooltip-open tooltip-top pt-1 mt-8" },
            "data-tip": translate!(i18, "errors.password_validation"),
            label { class: "input input-bordered flex items-center gap-2",
                svg {
                    "fill": "currentColor",
                    "xmlns": "http://www.w3.org/2000/svg",
                    "viewBox": "0 0 16 16",
                    class: "w-4 h-4 opacity-70",
                    path {
                        "fill-rule": "evenodd",
                        "d": "M14 6a4 4 0 0 1-4.899 3.899l-1.955 1.955a.5.5 0 0 1-.353.146H5v1.5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1-.5-.5v-2.293a.5.5 0 0 1 .146-.353l3.955-3.955A4 4 0 1 1 14 6Zm-4-2a.75.75 0 0 0 0 1.5.5.5 0 0 1 .5.5.75.75 0 0 0 1.5 0 2 2 0 0 0-2-2Z",
                        "clip-rule": "evenodd"
                    }
                }
                input { r#type: "password", name: "password", value: password, class: "grow",
                    placeholder: translate!(i18, "messages.password"),
                    oninput: move |event| input_update(event, password)
                }
            }
        }
        if validate_login() && validate_password() && error().is_empty() && !in_progress() {
            button { class: "btn btn-neutral btn-outline w-fit self-center mt-2",
                prevent_default: "onclick",
                onclick: move |_| {
                    in_progress.set(true);
                    app_state.service.sign_in(login, password, Some(error))
                },
                { translate!(i18, "messages.sign_in") }
            }
        } else if !error().is_empty() {
            div {
                prevent_default: "onclick",
                onclick: drop_error,
                MessageBoxComponent { kind: MessageBoxComponentKind::Error, message: error.read().clone() }
            }
        } else if in_progress() {
            div { class: "flex flex-row gap-4 w-fit self-center",
                span { class: "loading loading-spinner loading-md" }
                span { { translate!(i18, "messages.sign_in") } "..." }
            }
        }
    }
}