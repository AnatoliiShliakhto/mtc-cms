use super::*;

pub fn InitBox() -> Element {
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();
    let api_client = use_api_client();

    let busy = use_init_busy();

    let submit = move |event: Event<FormData>| {
        let url: Cow<'static, str> = url!(API_MIGRATE);
        let json_obj = json!({
            "login": event.get_str("login"),
            "password": event.get_str("password"),
        });

        spawn(async move {
            match api_client()
                .post(&*url)
                .json(&json_obj)
                .send()
                .await
                .consume()
                .await {
                Ok(_) => {
                    navigator().push(Route::SignIn {});
                },
                Err(e) =>
                    message_box_task.send(MessageBoxAction::Error(e.message())),
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
                                if busy() {
                                    div {
                                        class: "flex flex-nowrap gap-4 \
                                        self-center justify-center items-center",
                                        span {
                                            class: "loading loading-spinner loading-md"
                                        }
                                        span {
                                            { t!("action-init") } "..."
                                        }
                                    }
                                } else {
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
}