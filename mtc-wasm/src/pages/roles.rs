use super::*;

/// Component for displaying and managing roles.
///
/// This component presents a table with a list of roles, showing their slugs and titles.
/// It retrieves the roles data from the API and updates its state with the response.
/// Users can navigate to a specific role's page by clicking on its entry in the table.
/// The component includes a button for creating a new role, visible only to users with
/// the appropriate permission.
///
/// # Permissions
///
/// - Requires [`PERMISSION_ROLES_READ`] to view the roles.
/// - Requires [`PERMISSION_ROLES_WRITE`] to create a new role.
///
/// # Errors
///
/// - Displays an error message if the API response is null.
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