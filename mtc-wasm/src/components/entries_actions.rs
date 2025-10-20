use super::*;

/// A component to render the actions for a list of entries.
///
/// This component is expected to be used with a `Resource` as its first prop,
/// which is used to restart the future when the refresh button is clicked.
///
/// The second prop is an optional `Route` that is used to navigate to a route
/// when the create button is clicked. The third prop is an optional string
/// representing the permission required to show the create button.
///
/// The component will render a refresh button that restarts the future when
/// clicked, and a create button that navigates to the given route when clicked.
/// The create button is only visible if the user has the given permission.
///
/// The component will also render a div with the `action-right-panel` class
/// that contains the buttons.
#[component]
pub fn EntriesActions<T: PartialEq + 'static>(
    #[props] future: Resource<T>,
    #[props] route: Option<Route>,
    #[props] permission: Option<String>,
    #[props(default)] extra_buttons: Vec<Element>,
) -> Element {
    let auth = state!(auth);

    rsx! {
        div {
            class: "action-right-panel top-24 group join join-vertical \
            opacity-50 xl:opacity-100 hover:opacity-100",
            onclick: move |event| event.stop_propagation(),
            button {
                class: "hover:btn-primary join-item",
                onclick: move |_| future.restart(),
                Icon { icon: Icons::Refresh, class: "size-8" }
                span {
                    class: "opacity-0 group-hover:opacity-100",
                    { t!("action-refresh") }
                }
            }
            if let Some(route) = route {
                if auth.has_permission(
                    &permission.unwrap_or(ROLE_ADMINISTRATOR.into())
                ) {
                    button {
                        class: "hover:btn-accent join-item",
                        onclick: move |_| { navigator().push(route.clone()); },
                        Icon { icon: Icons::Plus, class: "size-8" }
                        span {
                            class: "opacity-0 group-hover:opacity-100",
                            { t!("action-create") }
                        }
                    }
                }
            }
            {extra_buttons.into_iter()}
        }
    }
}
