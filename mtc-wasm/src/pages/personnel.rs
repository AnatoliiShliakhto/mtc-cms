use super::*;

#[component]
pub fn Personnel() -> Element {
    breadcrumbs!("menu-personnel");
    check_permission!(PERMISSION_USERS_READ);
    let is_admin = use_auth_state()().is_admin();
    let mut confirmation = use_signal(|| false);

    let columns = use_personnel_columns();
    let column_actions = columns.actions;
    let column_login = columns.login;
    let column_rank = columns.rank;
    let column_name = columns.name;
    let column_password = columns.password;
    let column_group = columns.group;
    let column_access = columns.access;

    let mut users = use_personnel().users;

    let state_roles = use_app_state().roles;
    let state_groups = use_app_state().groups;

    let mut delete = move |login: &str| {
        users.write().remove(login);
    };

    let submit = move |event: Event<FormData>| {
        if !confirmation() { return }
        confirmation.set(false);
        let logins = users()
            .iter()
            .map(|(login, _)| login.clone())
            .collect::<Vec<Cow<'static, str>>>();
        let payload = json!({
            "block": event.get_bool("block"),
            "reassign": event.get_bool("reassign"),
            "recreate": event.get_bool("recreate"),
            "roles": event.get_str_array("roles").unwrap_or(vec![]),
            "group": event.get_str("group"),
            "logins": logins
        });

        spawn(async move {
            let response = value_request!(url!(API_USERS), payload);
            let Some(user_details_dto) =
                response.self_obj::<Vec<UserDetailsDto>>() else { return };
            use_personnel_assign_details(user_details_dto);
        });
    };

    rsx!{
        section {
            class: "w-full flex-grow xl:pr-16",
            if is_admin {
                details {
                    class: "collapse collapse-arrow bg-base-200 rounded",
                    summary {
                        class: "collapse-title font-medium",
                        { t!("caption-personnel-controls") }
                    }
                    div {
                        class: "collapse-content p-2",
                        form {
                            class: "flex flex-wrap w-full px-4 sm:px-0",
                            id: "personnel-form",
                            autocomplete: "off",
                            onsubmit: submit,

                            FormEntriesField {
                                name: "roles",
                                title: "field-roles",
                                items: vec![],
                                entries: state_roles()
                            }
                            div {
                                class: "grid w-full gap-5 items-end",
                                class: "grid-cols-2 sm:grid-cols-4 lg:grid-cols-6",
                                FormCheckBoxField {
                                    name: "reassign",
                                    title: "field-reassign-users-roles",
                                }
                                FormCheckBoxField {
                                    name: "recreate",
                                    title: "field-recreate-passwords",
                                }
                                FormCheckBoxField {
                                    name: "block",
                                    title: "field-block",
                                }
                                div {
                                    class: "w-full col-span-2",
                                    FormSelectField {
                                        name: "group",
                                        title: "field-group",
                                        selected: "",
                                        items: state_groups(),
                                    }
                                }
                                if confirmation() {
                                    button {
                                        class: "btn btn-success",
                                        { t!("action-process") }
                                    }
                                } else {
                                    button {
                                        class: "btn",
                                        onclick: move |event| {
                                            confirmation.set(true);
                                            event.stop_propagation()
                                        },
                                        { t!("action-confirm") }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            table {
                class: "entry-table table-xs min-w-[750px] mt-6",
                thead {
                    tr {
                        if column_actions() {
                            th { class: "w-10" }
                        }
                        th { class: "w-8" }
                        if column_login() {
                            th {
                                class: "w-28",
                                { t!("field-login") }
                            }
                        }
                        if column_rank() {
                            th {
                                class: "w-32",
                                { t!("field-rank") }
                            }
                        }
                        if column_name() {
                            th {
                                class: "min-w-36 text-wrap",
                                { t!("field-name") }
                            }
                        }
                        if column_password() {
                            th {
                                class: "w-32",
                                { t!("field-password") }
                            }
                        }
                        if column_group() {
                            th {
                                class: "min-w-36",
                                { t!("field-group") }
                            }
                        }
                        if column_access() {
                            th {
                                class: "w-32",
                                { t!("field-access") }
                            }
                        }
                    }
                }
                tbody {
                    for (login, user) in users() {{
                        let login_cloned = login.clone();
                        let id = user.id.clone();

                        rsx! {
                            tr {
                                onclick: move |_| {
                                    if id.is_empty() { return }
                                    navigator()
                                    .push(route!(API_ADMINISTRATOR, API_USER, &id));
                                },
                                if column_actions() {
                                    td {
                                        class: "p-1",
                                        onclick: move |event| event.stop_propagation(),
                                        button {
                                            class: "btn btn-xs btn-ghost hover:text-error",
                                            onclick: move |_| delete(&login_cloned),
                                            Icon {
                                                icon: Icons::Close,
                                                class: "size-4"
                                            }
                                        }
                                    }
                                }
                                td {
                                    class: "p-1 justify-items-center",
                                    match user.state {
                                        UserState::Active => rsx! {
                                            Icon {
                                                icon: Icons::UserCheck,
                                                class: "size-4 text-success"
                                            }
                                        },
                                        UserState::Inactive => rsx! {
                                            Icon {
                                                icon: Icons::Ban,
                                                class: "size-4 text-error"
                                            }
                                        },
                                        _ => rsx! {
                                            Icon {
                                                icon: Icons::Incognito,
                                                class: "size-4 text-neutral"
                                            }
                                        },
                                    }
                                }
                                if column_login() {
                                    td {
                                        { login }
                                    }
                                }
                                if column_rank() {
                                    td {
                                        { user.rank }
                                    }
                                }
                                if column_name() {
                                    td {
                                        { user.name }
                                    }
                                }
                                if column_password() {
                                    td {
                                        { user.password }
                                    }
                                }
                                if column_group() {
                                    td {
                                        { user.group }
                                    }
                                }
                                if column_access() {
                                    td {
                                        if let Some(access) = user.last_access {
                                            { access.format("%d/%m/%Y").to_string() }
                                            " (" { user.access_count.to_string() } ")"
                                        }
                                    }
                                }
                            }
                        }
                    }}
                }
            }
        }
        PersonnelActions {}
        PersonnelColumnsChooser {}
    }
}