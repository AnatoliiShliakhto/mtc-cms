use super::*;

#[component]
pub fn UserEdit(
    #[props(into)]
    id: String,
) -> Element {
    let id = use_memo(use_reactive!(|id| id));

    breadcrumbs!("menu-users");
    check_permission!(PERMISSION_USERS_READ);

    let future = value_future!(url!(API_USER, &id()));
    let response = future.suspend()?;
    check_response!(response, future);

    let submit = move |event: Event<FormData>| {
        let payload = json!({
            "id": event.get_str("id"),
            "login": event.get_str("login"),
            "password": event.get_str("password"),
            "group": event.get_str("group"),
            "blocked": event.get_bool("blocked"),
            "roles": event.get_str_array("roles").unwrap_or(vec![])
        });

        spawn(async move {
            if post_request!(url!(API_USER), payload) {
                navigator().replace(route!(API_ADMINISTRATOR, API_USERS));
            }
        });
    };

    let delete = move |event: MouseEvent| {
        spawn(async move {
            if delete_request!(url!(API_USER, &id())) {
                navigator().replace(route!(API_ADMINISTRATOR, API_USERS));
            }
        });
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
                    initial_value: response().key_string("id")
                }
                FormTextField {
                    name: "login",
                    title: "field-login",
                    required: true,
                    initial_value: response().key_string("login")
                }
                FormTextField {
                    name: "password",
                    title: "field-password",
                }
                FormSelectField {
                    name: "group",
                    title: "field-group",
                    selected: response().key_string("group").unwrap_or_default(),
                    items: response().key_obj::<Vec<Entry>>("groups_set").unwrap_or_default(),
                }
                FormEntriesField {
                    name: "roles",
                    title: "field-roles",
                    items: response().key_obj::<Vec<Cow<'static, str>>>("roles")
                    .unwrap_or_default(),
                    entries: response().key_obj::<Vec<Entry>>("roles_set")
                    .unwrap_or_default()
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
                form: "user-edit-form",
                permission: PERMISSION_USERS_WRITE,
            }
        } else {
            EditorActions {
                form: "user-edit-form",
                delete_handler: delete,
                permission: PERMISSION_USERS_WRITE,
            }
        }
        UserBlockAction {
            checked: response().key_bool("blocked").unwrap_or_default(),
        }
    }
}