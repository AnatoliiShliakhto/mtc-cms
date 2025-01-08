use super::*;

/// Page for showing and managing all users.
///
/// Shows a table with the login, rank, name and group of each user.
///
/// The component also provides a button for deleting each user.
///
/// The component is only visible if the user has the permission for viewing
/// roles.
///
/// The component also provides an action button for creating new users,
/// which is only visible if the user has the permission for writing roles.
#[component]
pub fn Users() -> Element {
    breadcrumbs!("menu-users");
    check_role!(ROLE_ADMINISTRATOR);
    check_permission!(PERMISSION_USERS_READ);

    let mut search = use_signal(String::new);
    let mut archive = use_signal(|| false);
    let personnel = state!(personnel);

    let future = value_future!(url!(API_USERS, &search(), &archive().to_string()));
    let response = future.suspend()?;
    check_response!(response, future);

    rsx! {
        section {
            class: "w-full flex-grow xl:pr-16",
            form {
                class: "w-full mb-6 pr-16 xl:pr-0",
                autocomplete: "off",
                onsubmit: move |event| {
                    search.set(event.get_str("login").unwrap_or_default().to_string());
                    archive.set(event.get_bool("archive"));
                },
                label {
                    class: "input input-bordered input-sm flex grow",
                    class: "mx-2 sm:mx-4 items-center gap-2",
                    input {
                        class: "grow peer",
                        style: "max-width: inherit; width: 100%",
                        r#type: "search",
                        name: "login",
                        placeholder: &*t!("message-search"),
                    }
                    div {
                        class: "relative -right-3 join",
                        label {
                            class: "swap join-item",
                            input {
                                r#type: "checkbox",
                                name: "archive"
                            }
                            div {
                                class: "swap-on",
                                Icon {
                                    icon: Icons::UserUp,
                                    class: "size-6 text-info"
                                }
                            }
                            div {
                                class: "swap-off",
                                Icon {
                                    icon: Icons::UserCheck,
                                    class: "size-6 text-success"
                                }
                            }
                        }
                        button {
                            class: "btn btn-sm btn-ghost join-item",
                            Icon { icon: Icons::Search, class: "size-6 text-primary" }
                        }
                    }
                }
            }

            table {
                class: "entry-table",
                thead {
                    tr {
                        th { class: "w-8" }
                        th { { t!("field-login") } }
                        th { { t!("field-rank") } }
                        th {
                            class: "text-wrap",
                            { t!("field-name") }
                        }
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
                        let details = personnel
                        .get(&user.slug)
                        .unwrap_or(&UserDetails::default())
                        .clone();

                        rsx! {
                            tr {
                                onclick: move |_| {
                                    navigator()
                                    .push(route!(API_ADMINISTRATOR, API_USER, &id));
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
                                    { details.rank.as_ref() }
                                }
                                td {
                                    { details.name.as_ref() }
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
                    route: route!(API_ADMINISTRATOR, API_USER, ID_CREATE),
                    permission: PERMISSION_USERS_WRITE,
                }
            }
        }
    }
}