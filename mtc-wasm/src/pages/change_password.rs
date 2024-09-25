use super::*;

pub fn ChangePassword() -> Element {
    build_breadcrumbs("menu-settings");

    if !use_auth_state()().is_authenticated() {
        navigator().push(Route::SignIn {});
        return rsx! { Loading {} }
    }
    
    let busy = use_init_busy();

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
                            { t!("message-change-password-from-header") } 
                        }
                        p { 
                            class: "py-6", 
                            { t!("message-change-password-from-announcement") } 
                        }
                    }
                    div { 
                        class: "card w-full max-w-sm shrink-0 border input-bordered rounded",
                        form { 
                            class: "card-body",
                            id: "password-form",
                            autocomplete: "off",
                            onsubmit: change_password_task,

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
                                if busy() {
                                    div { 
                                        class: "flex flex-nowrap gap-4 \
                                        self-center justify-center items-center",
                                        span { 
                                            class: "loading loading-spinner loading-md" 
                                        }
                                        span { 
                                            { t!("action-processing") } "..." 
                                        }
                                    }
                                } else {
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
}