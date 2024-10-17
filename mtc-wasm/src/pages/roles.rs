use super::*;

#[component]
pub fn Roles() -> Element {
    breadcrumbs!("menu-roles");
    check_permission!(PERMISSION_ROLES_READ);

    let future = value_future!(url!(API_ROLES));
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
                    for role in response()
                    .self_obj::<Vec<Entry>>()
                    .unwrap_or_default().iter() {{
                        let id = role.id.to_owned();

                        rsx! {
                            tr {
                                onclick: move |_| {
                                    navigator()
                                    .push(Route::RoleEdit { id: id.to_string() });
                                },
                                td {
                                    { role.slug.as_ref() }
                                }
                                td {
                                    { role.title.as_ref() }
                                }
                            }
                        }
                    }}
                }
                EntriesActions {
                    future,
                    route: Route::RoleEdit { id: ID_CREATE.to_string() },
                    permission: PERMISSION_ROLES_WRITE,
                }
            }
        }
    }
}