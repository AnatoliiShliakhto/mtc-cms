use super::*;

#[component]
pub fn Users() -> Element {
    breadcrumbs!("menu-users");
    check_permission!(PERMISSION_USERS_READ);

    let future = value_future!(url!(API_USERS));
    let response = future.suspend()?;
    check_response!(response, future);

    rsx! {
        section {
            class: "w-full flex-grow xl:pr-16",
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
                    for user in response()
                    .self_obj::<Vec<Entry>>()
                    .unwrap_or_default().iter() {{
                        let id = user.id.to_owned();
                        let blocked = user.variant.clone()
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
                                    { user.slug.as_ref() }
                                }
                                td {
                                    { user.title.as_ref() }
                                }
                            }
                        }
                    }}
                }
                EntriesActions {
                    future,
                    route: Route::UserEdit { id: ID_CREATE.to_string() },
                    permission: PERMISSION_USERS_WRITE,
                }
            }
        }
    }
}