use super::*;

/// A component for changing the user's password.
///
/// This component is only available if the user is authenticated.
///
/// It contains a form with three fields: `current-password`, `new-password` and
/// `password-confirmation`. The `current-password` and `new-password` fields are
/// required and must be filled in. The `password-confirmation` field must match
/// the `new-password` field.
///
/// When the form is submitted, a PATCH request is sent to the server with the
/// current password and the new password. If the request is successful, a
/// success dialog is displayed. If the request fails, an error dialog is
/// displayed.
#[component]
pub fn ChangePassword() -> Element {
    breadcrumbs!("menu-change-password");

    if !state!(auth).is_authenticated() {
        navigator().push(route!(API_AUTH, API_SIGN_IN));
        return rsx! { Loading {} }
    }

    let submit = move |event: Event<FormData>| {
        let current_password =
            event.get_str("current-password").unwrap_or_default();
        let new_password =
            event.get_str("new-password").unwrap_or_default();
        let password_confirmation =
            event.get_str("password-confirmation").unwrap_or_default();

        if new_password.ne(&password_confirmation) {
            error_dialog!("error-password-not-match");
            return
        }

        let payload = json!({
            "current_password": current_password,
            "new_password": new_password
        });

        spawn(async move {
            if patch_request!(url!(API_AUTH), payload) {
                success_dialog!("message-password-changed")
            }
        });
    };

    rsx!{
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
                            { t!("message-change-password-form-header") }
                        }
                        p { 
                            class: "py-6", 
                            { t!("message-change-password-form-announcement") }
                        }
                    }
                    div { 
                        class: "card w-full max-w-sm shrink-0 border input-bordered rounded",
                        form { 
                            class: "card-body",
                            id: "password-form",
                            autocomplete: "off",
                            onsubmit: submit,

                            FormTextField {
                                r#type: "password",
                                name: "current-password",
                                title: "field-current-password",
                                required: true
                            }
                            FormTextField {
                                r#type: "password",
                                name: "new-password",
                                title: "field-new-password",
                                required: true
                            }
                            FormTextField {
                                r#type: "password",
                                name: "password-confirmation",
                                title: "field-password-confirmation",
                                required: true
                            }

                            div { 
                                class: "form-control mt-6",
                                button {
                                    class: "btn btn-primary",
                                    r#type: "submit",
                                    Icon { icon: Icons::Lock, class: "size-6" }
                                    { t!("action-change-password") }
                                }
                            }
                        }
                    }    
                }
            }
        }
    }
}