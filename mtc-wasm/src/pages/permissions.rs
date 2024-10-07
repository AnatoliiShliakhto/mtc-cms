use super::*;

pub fn Permissions() -> Element {
    let auth_state = use_auth_state();
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();
    let api_task = use_coroutine_handle::<ApiRequestAction>();

    page_init!("menu-permissions", PERMISSION_ROLES_READ, auth_state);

    let future =
        use_resource(move || async move {
            request_fetch_task(url!(API_PERMISSIONS)).await
        });
    let response = future.suspend()?;
    if response().is_null() { fail!(future) }

    rsx! {
        section { 
            class: "w-full flex-grow sm:pr-16",
            table { 
                class: "entry-table",
                thead {
                    tr {
                        th { class: "w-12" }
                        th { { t!("field-slug") } }
                    }
                }
                tbody {
                    for item in response().as_array().unwrap_or(&vec![]).iter() {{
                        let permission = item.to_owned();

                        rsx! {
                            tr {
                                td {
                                    if auth_state().has_permission(PERMISSION_ROLES_DELETE) {
                                        button {
                                            class: "btn btn-xs btn-ghost",
                                            onclick: move |_| {
                                                if let Some(permission) = permission.as_str() {
                                                    api_task.send(
                                                        ApiRequestAction::DeleteThenMessage(
                                                            url!(API_PERMISSION,  permission),
                                                            None,
                                                        )
                                                    )
                                                }
                                            },
                                            Icon { icon: Icons::Close, class: "size-4 text-error" }
                                        }
                                    }
                                }
                                td {
                                    { item.as_str() }
                                }
                            }
                        }
                    }}
                }
                EntriesActions {
                    future,
                    route: Route::PermissionCreate {}.to_string(),
                    permission: PERMISSION_ROLES_WRITE,
                }
            }
        }
    }
}