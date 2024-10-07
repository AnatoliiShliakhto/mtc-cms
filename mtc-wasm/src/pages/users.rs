use super::*;

pub fn Users() -> Element {
    let auth_state = use_auth_state();
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();

    page_init!("menu-users", PERMISSION_USERS_READ, auth_state);

    let future =
        use_resource(move || async move {
            request_fetch_entries_task(url!(API_USERS)).await
        });
    let response = future.suspend()?;
    if response().is_none() { fail!(future) }

    rsx! {
        section {
            class: "w-full flex-grow sm:pr-16",
            table {
                class: "entry-table",
                thead {
                    tr {
                        th { class: "w-8" }
                        th { { t!("field-login") } }
                        th { { t!("field-group") } }
                    }
                }
                tbody {
                    for item in response().unwrap_or(vec![]).iter() {{
                        let id = item.id.to_owned();
                        let blocked = item.variant.clone()
                            .unwrap_or_default()
                            .as_bool()
                            .unwrap_or_default();

                        rsx! {
                            tr {
                                onclick: move |_| {
                                    navigator()
                                    .push(Route::UserEdit { id: id.to_string() });
                                },
                                td {
                                    if blocked {
                                        Icon {
                                            icon: Icons::Ban,
                                            class: "w-4 text-error",
                                        }
                                    }
                                }
                                td {
                                    { item.slug.as_ref() }
                                }
                                td {
                                    { item.title.as_ref() }
                                }
                            }
                        }
                    }}
                }
                EntriesActions {
                    future,
                    route: Route::UserEdit { id: ID_CREATE.to_string() }.to_string(),
                    permission: PERMISSION_USERS_WRITE,
                }
            }
        }
    }
}