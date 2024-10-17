use super::*;

#[component]
pub fn MenuItem(
    #[props(into)]
    route: Route,
    #[props]
    permission: Option<String>,
    #[props]
    children: Option<Element>,
) -> Element {

    match permission {
        Some(permission) => rsx! {
            if use_auth_state()().has_permission(&permission) {
                li {
                    Link { 
                        to: route,
                        onclick: move |_| {
                            use_search_engine_drop();
                            use_menu_state().set(false)
                        },
                        { children }
                    }
                }
            }
        },
        _ => rsx! {
            li {
                Link { 
                    to: route,
                    onclick: move |_| {
                        use_search_engine_drop();
                        use_menu_state().set(false)
                    },
                    { children }
                }
            }
        },
    }
}