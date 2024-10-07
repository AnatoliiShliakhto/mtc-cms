use super::*;

#[component]
pub fn RoleEdit(
    #[props(into)]
    id: String,
) -> Element {
    let id = use_memo(use_reactive!(|id| id));

    let message_box_task = use_coroutine_handle::<MessageBoxAction>();
    let api_task = use_coroutine_handle::<ApiRequestAction>();
    let auth_state = use_auth_state();

    page_init!("menu-roles", PERMISSION_ROLES_READ, auth_state);

    let future =
        use_resource(move || async move {
            request_fetch_task(url!(API_ROLE, &id())).await
        });

    let response = future.suspend()?;
    if response().is_null() { fail!(future) }

    let submit = move |event: Event<FormData>| {
        api_task.send(ApiRequestAction::PostThenBack(
            url!(API_ROLE),
            Some(json!({
                "id": event.get_str("id"),
                "slug": event.get_str("slug"),
                "title": event.get_str("title"),
                "user_access_all": event.get_bool("user_access_all"),
                "user_access_level": event.get_i64("user_access_level").unwrap_or(999),
                "permissions": event.get_str_array("permissions")
            })),
        ))
    };

    let delete = move |event: MouseEvent| {
        api_task.send(ApiRequestAction::DeleteThenBack(
            url!(API_ROLE, &id()),
            None,
        ))
    };

    rsx! {
        section {
            class: "flex grow select-none flex-row gap-6 px-3 pr-20 sm:pr-16",
            form {
                class: "flex grow flex-col items-center gap-3",
                id: "role-edit-form",
                autocomplete: "off",
                onsubmit: submit,

                input {
                    r#type: "hidden",
                    name: "id",
                    initial_value: response().get_string("id")
                }
                FormTextField {
                    name: "slug",
                    title: "field-slug",
                    pattern: SLUG_PATTERN,
                    required: true,
                    initial_value: response().get_string("slug")
                }
                FormTextField {
                    name: "title",
                    title: "field-title",
                    pattern: TITLE_PATTERN,
                    required: true,
                    initial_value: response().get_string("title")
                }
                FormNumField {
                    name: "user_access_level",
                    title: "field-access-level",
                    min: "1",
                    max: "999",
                    step: "1",
                    required: true,
                    initial_value: response()
                        .get_i64("user_access_level").unwrap_or(999).to_string()
                }
                FormToggleField {
                    name: "user_access_all",
                    title: "field-user-all-access",
                    checked: response()
                        .get_bool("user_access_all").unwrap_or_default()
                }
                FormEntriesField {
                    name: "permissions",
                    title: "field-permissions",
                    items: response().get_str_array("permissions").unwrap_or(vec![]),
                    entries: response().get_entries("permissions_set").unwrap_or(vec![]),
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
                form: "role-edit-form",
                permission: PERMISSION_ROLES_WRITE,
            }
        } else {
            EditorActions {
                form: "role-edit-form",
                delete_event: delete,
                permission: PERMISSION_ROLES_WRITE,
            }
        }
    }
}