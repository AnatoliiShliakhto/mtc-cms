use super::*;

pub fn SignIn() -> Element {
    build_breadcrumbs("menu-sign-in");

    if use_auth_state()().is_authenticated() {
        navigator().push(Route::ChangePassword {});
        return rsx! { Loading {} }
    }

    let busy = use_init_busy();

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
                            { t!("message-login-from-header") }
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
                            onsubmit: sign_in_task,

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
                                if busy() {
                                    div {
                                        class: "flex flex-nowrap gap-4 \
                                        self-center justify-center items-center",
                                        span {
                                            class: "loading loading-spinner loading-md"
                                        }
                                        span {
                                            { t!("action-sign-in") } "..."
                                        }
                                    }
                                } else {
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
}