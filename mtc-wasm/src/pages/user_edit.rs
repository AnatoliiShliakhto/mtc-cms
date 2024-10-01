use super::*;

#[component]
pub fn UserEdit(
    #[props]
    id: ReadOnlySignal<String>
) -> Element {
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();
    let api_task = use_coroutine_handle::<ApiRequestAction>();
    let auth_state = use_auth_state();
    page_init!("menu-users", PERMISSION_USERS_READ, auth_state);

    let future =
        use_resource(move || async move {
            request_fetch_task(url!("user", &id())).await
        });

    let response = future.suspend()?;
    if response().is_null() { fail!(future) }

    let submit = move |event: Event<FormData>| {
        api_task.send(ApiRequestAction::PostThenBack(
            url!("user"),
            Some(json!({
                "id": event.get_str("id"),
                "login": event.get_str("login"),
                "password": event.get_str("password"),
                "group": event.get_str("group"),
                "blocked": event.get_bool("blocked"),
                "roles": event.get_str_array("roles").unwrap_or(vec![]),
            })),
        ))
    };

    let delete = move |event: MouseEvent| {
        api_task.send(ApiRequestAction::DeleteThenBack(
            url!("user", &id()),
            None,
        ))
    };

    rsx! {
        section {
            class: "flex grow select-none flex-row gap-6 px-3 pr-20 sm:pr-16",
            form {
                class: "flex grow flex-col items-center gap-3",
                id: "user-edit-form",
                autocomplete: "off",
                onsubmit: submit,

                input {
                    r#type: "hidden",
                    name: "id",
                    initial_value: response().get_string("id")
                }
                FormTextField {
                    name: "login",
                    title: "field-login",
                    required: true,
                    initial_value: response().get_string("login")
                }
                FormTextField {
                    name: "password",
                    title: "field-password",
                }
                FormSelectField {
                    name: "group",
                    title: "field-group",
                    selected: response().get_string("group").unwrap_or_default(),
                    items: response().get_entries("groups_set").unwrap_or(vec![]),
                }
                FormEntriesField {
                    name: "roles",
                    title: "field-roles",
                    items: response().get_str_array("roles").unwrap_or(vec![]),
                    entries: response().get_entries("roles_set").unwrap_or(vec![]),
                }
            }
        }
        EntryInfoBox {
            created_by: response().get_string("created_by"),
            created_at: response().get_datetime("created_at"),
            updated_by: response().get_string("updated_by"),
            updated_at: response().get_datetime("updated_at"),
        }
        if id().eq(ID_CREATE) {
            EditorActions {
                form: "user-edit-form",
                permission: PERMISSION_USERS_WRITE,
            }
        } else {
            EditorActions {
                form: "user-edit-form",
                delete_event: delete,
                permission: PERMISSION_USERS_WRITE,
            }
        }
        UserBlockAction {
            checked: response().get_bool("blocked").unwrap_or_default(),
        }
    }
}