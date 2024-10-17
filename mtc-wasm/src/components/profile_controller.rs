use super::*;

#[component]
pub fn ProfileController() -> Element {
    let auth_state =  use_auth_state();

    if !auth_state().is_authenticated() {
        return rsx! {
            Link {
                class: "btn btn-ghost join-item",
                onclick: move |_| use_search_engine_drop(),
                to: Route::SignIn {},
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
                        onclick: move |_| use_search_engine_drop(),
                        to: Route::ChangePassword {},
                        Icon { icon: Icons::Settings, class: "size-6" }
                        { t!("menu-settings") }
                    }
                }
                if auth_state().is_admin() {
                    li {
                        Link {
                            onclick: move |_| use_search_engine_drop(),
                            to: Route::Administrator {},
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