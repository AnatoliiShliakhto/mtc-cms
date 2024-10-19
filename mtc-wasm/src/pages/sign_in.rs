use super::*;

#[component]
pub fn SignIn() -> Element {
    let sync = use_coroutine_handle::<SyncAction>();

    breadcrumbs!("menu-sign-in");

    let submit = move |event: Event<FormData>| {
        let payload = json!({
            "login": event.get_str("login"),
            "password": event.get_str("password")
        });
        spawn(async move {
            if post_request!(url!(API_AUTH), payload) {
                sync.send(SyncAction::RefreshState("".into()));

                if navigator().can_go_back() {
                    navigator().go_back()
                } else {
                    navigator().replace(route!());
                }
            }
        });
    };

    rsx! {
        div {
            class: "div-centered",
            div {
                class: "hero",
                div {
                    class: "hero-content flex-col lg:flex-row-reverse",
                    div {
                        class: "text-center lg:text-left w-full sm:max-w-sm",
                        h1 {
                            class: "text-3xl font-bold",
                            { t!("message-login-form-header") }
                        }
                        p {
                            class: "py-6",
                            { t!("message-login-form-announcement") }
                        }
                    }
                    div {
                        class: "card w-full max-w-sm shrink-0 border input-bordered rounded",
                        form {
                            class: "card-body",
                            autocomplete: "off",
                            onsubmit: submit,

                            FormTextField {
                                name: "login",
                                title: "field-login",
                                required: true
                            }
                            FormTextField {
                                r#type: "password",
                                name: "password",
                                title: "field-password",
                                required: true
                            }

                            div {
                                class: "form-control mt-6",
                                button {
                                    class: "btn btn-primary",
                                    r#type: "submit",
                                    Icon { icon: Icons::SignIn, class: "size-6" }
                                    { t!("action-sign-in") }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}