use std::collections::HashMap;
use std::sync::LazyLock;
use super::*;

static CHAR_MAP: LazyLock<HashMap<char, char>> = LazyLock::new(|| {
    [
        ('A', 'А'),
        ('B', 'В'),
        ('E', 'Е'),
        ('I', 'І'),
        ('C', 'С'),
        ('K', 'К'),
        ('L', 'І'),
        ('O', 'О'),
        ('P', 'Р'),
        ('T', 'Т'),
        ('H', 'Н'),
        ('M', 'М'),
        ('X', 'Х'),
    ].into_iter().collect()
});
fn latin_to_cyrillic(input: &str) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < input.len() {
        let ch = input[i..].chars().next().unwrap();
        if let Some(&cyr) = CHAR_MAP.get(&ch) {
            result.push(cyr);
        } else {
            result.push(ch);
        }
        i += ch.len_utf8();
    }
    result
}

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
            "login": event.get_str("login").map(|login| latin_to_cyrillic(&login.to_uppercase())),
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
                        class: "card bg-base-100 card-border w-full max-w-sm shrink-0 shadow-xl",
                        form {
                            class: "card-body pt-4",
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
                                class: "card-actions px-6 pb-8",
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
