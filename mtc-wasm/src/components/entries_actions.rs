use super::*;

#[component]
pub fn EntriesActions<T: PartialEq + 'static>(
    #[props]
    future: Resource<T>,
    #[props]
    route: Option<String>,
    #[props]
    permission: Option<String>,
) -> Element {
    let auth_state = use_auth_state();

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
                if auth_state().has_permission(
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
        }
    }
}