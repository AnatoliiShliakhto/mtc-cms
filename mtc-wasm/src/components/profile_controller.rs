use super::*;

/// A component that renders a profile controller for the user.
#[component]
pub fn ProfileController() -> Element {
    let auth = state!(auth);

    if !auth.is_authenticated() {
        return rsx! {
            Link {
                class: "btn btn-ghost join-item",
                onclick: move |_| state_fn!(search_engine_clear),
                to: route!(API_AUTH, API_SIGN_IN),
                Icon { icon: Icons::SignIn, class: "size-8 sm:size-6" }
            }
        }    
    }

    rsx! {
        div { 
            class: "dropdown dropdown-end dropdown-hover join-item",
            div { 
                tabindex: "0", 
                role: "button", 
                class: "btn btn-ghost join-item",
                Icon { icon: Icons::Person, class: "size-8 sm:size-6" }
            }
            ul { 
                tabindex: "0", 
                class: "w-52 rounded border p-2 shadow-md dropdown-content z-[1] \
                menu bg-base-100 input-bordered",
                "onclick": "document.activeElement.blur()",
                li {
                    Link {
                        onclick: move |_| state_fn!(search_engine_clear),
                        to: route!(API_AUTH, "change-password"),
                        Icon { icon: Icons::Lock, class: "size-6" }
                        { t!("menu-change-password") }
                    }
                }
                li {
                    Link {
                        onclick: move |_| state_fn!(search_engine_clear),
                        to: route!(API_AUTH, "linking-qr-code"),
                        Icon { icon: Icons::QrCode, class: "size-6" }
                        { t!("menu-linking-qr-code") }
                    }
                }
                if auth.is_admin() {
                    li {
                        Link {
                            onclick: move |_| state_fn!(search_engine_clear),
                            to: route!(API_ADMINISTRATOR),
                            Icon { icon: Icons::ShieldPerson, class: "size-6" }
                            { t!("menu-administrator") }
                        }
                    }
                }
                div { class: "divider my-0" }
                li {
                    a {
                        onclick: sign_out_task,
                        Icon { icon: Icons::SignOut, class: "size-6" }
                        { t!("menu-sign-out") }
                    }
                }
            }
        }
    }
}