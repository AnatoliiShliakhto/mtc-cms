use super::*;

/// The component displays a form to edit an existing group, or create a new one.
///
/// The component is protected by the [`PERMISSION_GROUPS_READ`] permission.
///
/// # Properties
///
/// * `id`: the ID of the group to edit, or [`ID_CREATE`] to create a new one.
#[component]
pub fn GroupEdit(
    #[props(into)]
    id: String,
) -> Element {
    let id = use_memo(use_reactive!(|id| id));

    breadcrumbs!("menu-groups");
    check_permission!(PERMISSION_GROUPS_READ);

    let future = value_future!(url!(API_GROUP, &id()));
    let response = future.suspend()?;
    check_response!(response, future);

    let submit = move |event: Event<FormData>| {
        let payload = json!({
            "id": event.get_str("id"),
            "slug": event.get_str("slug"),
            "title": event.get_str("title")
        });

        spawn(async move {
            if post_request!(url!(API_GROUP), payload) {
                navigator().replace(route!(API_ADMINISTRATOR, API_GROUPS));
            }
        });
    };

    let delete = move |event: Event<MouseData>| {
        spawn(async move {
            if delete_request!(url!(API_GROUP, &id())) {
                navigator().replace(route!(API_ADMINISTRATOR, API_GROUPS));
            }
        });
    };

    rsx! {
        section {
            class: "flex grow select-none flex-row gap-6 px-3 pr-20 sm:pr-16",
            form {
                class: "flex grow flex-col items-center gap-3",
                id: "group-edit-form",
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
                form: "group-edit-form",
                permission: PERMISSION_GROUPS_WRITE,
            }
        } else {
            EditorActions {
                form: "group-edit-form",
                delete_handler: delete,
                permission: PERMISSION_GROUPS_WRITE,
            }
        }
    }
}