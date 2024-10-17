use super::*;

#[component]
pub fn Groups() -> Element {
    breadcrumbs!("menu-groups");
    check_permission!(PERMISSION_GROUPS_READ);

    let future = value_future!(url!(API_GROUPS));
    let response = future.suspend()?;
    check_response!(response, future);

    rsx! {
        section {
            class: "w-full flex-grow xl:pr-16",
            table {
                class: "entry-table",
                thead {
                    tr {
                        th { { t!("field-slug") } }
                        th { { t!("field-title") } }
                    }
                }
                tbody {
                    for group in response()
                    .self_obj::<Vec<Entry>>()
                    .unwrap_or_default().iter() {{
                        let id = group.id.to_owned();

                        rsx! {
                            tr {
                                onclick: move |_| {
                                    navigator()
                                    .push(Route::GroupEdit { id: id.to_string() });
                                },
                                td {
                                    { group.slug.as_ref() }
                                }
                                td {
                                    { group.title.as_ref() }
                                }
                            }
                        }
                    }}
                }
                EntriesActions {
                    future,
                    route: Route::GroupEdit { id: ID_CREATE.to_string() },
                    permission: PERMISSION_GROUPS_WRITE,
                }
            }
        }
    }
}