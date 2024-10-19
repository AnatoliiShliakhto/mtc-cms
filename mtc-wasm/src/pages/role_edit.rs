use super::*;

#[component]
pub fn RoleEdit(
    #[props(into)]
    id: String,
) -> Element {
    let id = use_memo(use_reactive!(|id| id));

    breadcrumbs!("menu-roles");
    check_permission!(PERMISSION_ROLES_READ);

    let future = value_future!(url!(API_ROLE, &id()));
    let response = future.suspend()?;
    check_response!(response, future);

    let submit = move |event: Event<FormData>| {
        let payload = json!({
            "id": event.get_str("id"),
            "slug": event.get_str("slug"),
            "title": event.get_str("title"),
            "user_access_all": event.get_bool("user_access_all"),
            "user_access_level": event.get_i64("user_access_level").unwrap_or(999),
            "permissions": event.get_str_array("permissions")
        });

        spawn(async move {
            if post_request!(url!(API_ROLE), payload) {
                navigator().replace(route!(API_ADMINISTRATOR, API_ROLES));
            }
        });
    };

    let delete = move |event: MouseEvent| {
        spawn(async move {
            if delete_request!(url!(API_ROLE, &id())) {
                navigator().replace(route!(API_ADMINISTRATOR, API_ROLES));
            }
        });
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
                    initial_value: response().key_string("id")
                }
                FormTextField {
                    name: "slug",
                    title: "field-slug",
                    pattern: SLUG_PATTERN,
                    required: true,
                    initial_value: response().key_string("slug")
                }
                FormTextField {
                    name: "title",
                    title: "field-title",
                    pattern: TITLE_PATTERN,
                    required: true,
                    initial_value: response().key_string("title")
                }
                FormNumField {
                    name: "user_access_level",
                    title: "field-access-level",
                    min: "1",
                    max: "999",
                    step: "1",
                    required: true,
                    initial_value: response()
                    .key_i64("user_access_level")
                    .unwrap_or(999).to_string()
                }
                FormToggleField {
                    name: "user_access_all",
                    title: "field-user-all-access",
                    checked: response()
                        .key_bool("user_access_all").unwrap_or_default()
                }
                FormEntriesField {
                    name: "permissions",
                    title: "field-permissions",
                    items: response().key_obj::<Vec<Cow<'static, str>>>("permissions")
                    .unwrap_or_default(),
                    entries: response().key_obj::<Vec<Entry>>("permissions_set")
                    .unwrap_or_default(),
                }
            }
        }
        EntryInfoBox {
            created_by: response().key_string("created_by"),
            created_at: response().key_datetime("created_at"),
            updated_by: response().key_string("updated_by"),
            updated_at: response().key_datetime("updated_at"),
        }
        if id().eq(ID_CREATE) {
            EditorActions {
                form: "role-edit-form",
                permission: PERMISSION_ROLES_WRITE,
            }
        } else {
            EditorActions {
                form: "role-edit-form",
                delete_handler: delete,
                permission: PERMISSION_ROLES_WRITE,
            }
        }
    }
}