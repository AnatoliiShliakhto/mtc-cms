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
            if state!(auth).has_permission(&permission) {
                li {
                    Link { 
                        to: route,
                        onclick: move |_| {
                            state_fn!(search_engine_clear);
                            state!(set_menu, false)
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
                        state_fn!(search_engine_clear);
                        state!(set_menu, false)
                    },
                    { children }
                }
            }
        },
    }
}