use super::*;

#[component]
pub fn Roles() -> Element {
    breadcrumbs!("menu-roles");
    check_permission!(PERMISSION_ROLES_READ);

    let future = value_future!(url!(API_ROLES));
    let response = future.suspend()?;
    check_response!(response, future);

    state!(set_roles, response().self_obj::<Vec<Entry>>().unwrap_or_default());

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
                                    .push(route!(API_ADMINISTRATOR, API_ROLE, &id));
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
                    route: route!(API_ADMINISTRATOR, API_ROLE, ID_CREATE),
                    permission: PERMISSION_ROLES_WRITE,
                }
            }
        }
    }
}