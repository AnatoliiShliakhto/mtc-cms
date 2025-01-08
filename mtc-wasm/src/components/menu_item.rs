use super::*;

/// A menu item component that renders a list item (`li`) containing a link.
///
/// # Props
/// - `route`: The destination route for the link.
/// - `permission`: An optional permission string that, if provided, determines
///   whether the menu item is displayed. The item is displayed only if the user
///   has the specified permission.
/// - `children`: Optional child elements to be rendered inside the link.
///
/// The component checks the provided `permission`. If the user has the permission,
/// or if no permission is specified, it renders a list item with a clickable link
/// that clears the search engine state and closes the menu when clicked.
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