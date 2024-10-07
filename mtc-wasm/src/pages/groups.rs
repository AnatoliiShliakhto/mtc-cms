use super::*;

pub fn Groups() -> Element {
    let auth_state = use_auth_state();
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();

    page_init!("menu-groups", PERMISSION_GROUPS_READ, auth_state);

    let future =
        use_resource(move || async move {
            request_fetch_entries_task(url!(API_GROUPS)).await
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
                        th { { t!("field-slug") } }
                        th { { t!("field-title") } }
                    }
                }
                tbody {
                    for item in response().unwrap_or(vec![]).iter() {{
                        let id = item.id.to_owned();

                        rsx! {
                            tr {
                                onclick: move |_| {
                                    navigator()
                                    .push(Route::GroupEdit { id: id.to_string() });
                                },
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
                    route: Route::GroupEdit { id: ID_CREATE.to_string() }.to_string(),
                    permission: PERMISSION_GROUPS_WRITE,
                }
            }
        }
    }
}