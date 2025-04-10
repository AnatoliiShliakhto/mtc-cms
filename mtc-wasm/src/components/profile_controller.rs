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
            class: "dropdown dropdown-end block join-item",
            div { 
                tabindex: "0", 
                role: "button", 
                class: "btn btn-ghost join-item",
                Icon { icon: Icons::Person, class: "size-8 sm:size-6" }
            }
            ul { 
                tabindex: "0", 
                class: "dropdown-content bg-base-200 text-base-content rounded-box",
                class: "top-px max-h-[calc(100vh-11rem)] w-52 overflow-y-hidden",
                class: "border border-white/5 shadow-2xl outline-1 outline-black/5 mt-16 z-1",
                "onclick": "document.activeElement.blur()",
                li {
                    Link {
                        class: "btn w-full justify-start rounded-none",
                        onclick: move |_| state_fn!(search_engine_clear),
                        to: route!(API_AUTH, "change-password"),
                        Icon { icon: Icons::Lock, class: "size-6 mr-2 text-neutral" }
                        { t!("menu-change-password") }
                    }
                }
                li {
                    Link {
                        class: "btn w-full justify-start rounded-none",
                        onclick: move |_| state_fn!(search_engine_clear),
                        to: route!(API_AUTH, "linking-qr-code"),
                        Icon { icon: Icons::QrCode, class: "size-6 mr-2 text-neutral" }
                        { t!("menu-linking-qr-code") }
                    }
                }
                if auth.is_admin() {
                    li {
                        Link {
                            class: "btn w-full justify-start rounded-none",
                            onclick: move |_| state_fn!(search_engine_clear),
                            to: route!(API_ADMINISTRATOR),
                            Icon { icon: Icons::ShieldPerson, class: "size-6 mr-2 text-neutral" }
                            { t!("menu-administrator") }
                        }
                    }
                }
                div { class: "divider my-0" }
                li {
                    a {
                        class: "btn w-full justify-start rounded-none",
                        onclick: sign_out_task,
                        Icon { icon: Icons::SignOut, class: "size-6 mr-2 text-neutral" }
                        { t!("menu-sign-out") }
                    }
                }
            }
        }
    }
}