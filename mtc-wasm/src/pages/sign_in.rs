use super::*;

/// Displays a sign-in form for users to log in to the system.
///
/// If the user is already authenticated, this component redirects
/// to the main page.
///
/// The component renders a centered div with a hero section
/// containing a header, announcement message, and a card with a
/// sign-in form. The form contains login and password fields, and
/// a submit button. The form is submitted with a POST request to
/// the authentication API.
///
/// If the user is on an Android device, the component renders a
/// second button to scan a QR code for device linking.
#[component]
pub fn SignIn() -> Element {
    breadcrumbs!("menu-sign-in");

    if state!(auth).is_authenticated() {
        navigator().replace(route!(""));
        return rsx! { Loading {} }
    }

    let platform = state!(platform);

    let sync = use_coroutine_handle::<SyncAction>();

    let submit = move |event: Event<FormData>| {
        let payload = json!({
            "login": event.get_str("login"),
            "password": event.get_str("password")
        });
        spawn(async move {
            if !post_request!(url!(API_AUTH), payload) { return; };

            sync.send(SyncAction::RefreshState());

            if navigator().can_go_back() {
                navigator().go_back()
            } else {
                navigator().replace(route!());
            }
        });
    };

    if platform.eq("android") {
        eval(r#"
            try {
                await window.__TAURI__.barcodeScanner.cancel();
            } catch (error) {
                console.log(error);
            }
        "#);
    }

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
                            id: "sign-in-form",
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
                        }

                        if platform.eq("android") {
                            div {
                                class: "card-actions grid grid-cols-2 gap-10 px-8 pb-8",
                                button {
                                    form: "sign-in-form",
                                    class: "btn w-full btn-primary",
                                    Icon { icon: Icons::SignIn, class: "size-6" }
                                    { t!("action-sign-in") }
                                }
                                button {
                                    class: "btn w-full",
                                    onclick: move |_| {
                                        navigator().push(route!(API_AUTH, "qr-sign-in"));
                                    },
                                    Icon { icon: Icons::QrScan, class: "size-6" }
                                    { t!("action-scan-qr-code") }
                                }
                            }
                        } else {
                            div {
                                class: "card-actions px-8 pb-8",
                                button {
                                    form: "sign-in-form",
                                    class: "btn w-full btn-primary",
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
