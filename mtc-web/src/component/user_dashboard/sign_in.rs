use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::action::auth_action::AuthAction;
use crate::global_signal::APP_ERROR;
use crate::widget::message_box::{MessageBoxWidget, MessageBoxWidgetKind};

#[component]
pub fn SignIn() -> Element {
    let i18 = use_i18();

    let mut login = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    let validate_login = login.read().len().ge(&4);
    let validate_password = password.read().len().ge(&6);

    let mut error = APP_ERROR.signal();

    rsx! {
        div { class: "flex flex-col m-auto gap-6 p-10 rounded-lg my-10 shadow-lg hover:shadow-xl self-center min-w-96",
            div { class: if !validate_login && !login.read().is_empty() { "tooltip tooltip-open tooltip-top pt-1" },
                  "data-tip": translate!(i18, "messages.login_validation_tooltip"),
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
                        oninput: move |login_field_event| {
                            login.set(login_field_event.value());
                            error.set("".to_string());
                        }
                    }
                }
            }

            div { class: if !validate_password && !password.read().is_empty() { "tooltip tooltip-open tooltip-top pt-1" },
                  "data-tip": translate!(i18, "messages.password_validation_tooltip"),
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
                        oninput: move |password_field_event| {
                            password.set(password_field_event.value());
                            error.set("".to_string());
                        }
                    }
                }
            }
            if validate_login && validate_password && error.read().is_empty() {
                button { class: "btn btn-primary w-fit self-center",
                    onclick: move |_| {
                        spawn(async move {
                            use_coroutine_handle::<AuthAction>().send(AuthAction::SignIn(login.to_string(), password.to_string()));
                        });
                    },
                    { translate!(i18, "messages.sign_in") }
                }
            }
            if !error.read().is_empty() {
                div { class: "w-fit self-center",
                    onclick: move |_| {
                        spawn(async move {
                            error.set("".to_string());
                        });
                    },
                    MessageBoxWidget { kind: MessageBoxWidgetKind::Error, message: error.read().clone() }
                }
            }
        }
    }
}