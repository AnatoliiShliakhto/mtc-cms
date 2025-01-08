use super::*;

/// A component that renders a form to initialize the application.
///
/// The component is shown on the root route if the database is not initialized.
/// The form contains two fields: login and password. The form is submitted
/// to the [`API_MIGRATE`] endpoint, which initializes the database and
/// redirects to the [`API_AUTH`] endpoint if the initialization is successful.
///
/// The component styles the form according to the Tailwind CSS framework.
#[component]
pub fn InitBox() -> Element {

    let submit = move |event: Event<FormData>| {
        let payload = json!({
            "login": event.get_str("login"),
            "password": event.get_str("password")
        });
        spawn(async move {
            if post_request!(url!(API_MIGRATE), payload) {
                navigator().replace(route!(API_AUTH, API_SIGN_IN));
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
                            { t!("message-init-form-header") }
                        }
                        p {
                            class: "py-6",
                            { t!("message-init-form-announcement") }
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
                                    Icon { icon: Icons::Settings, class: "size-6" }
                                    { t!("action-init") }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}