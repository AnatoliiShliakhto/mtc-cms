use super::*;

#[component]
pub fn SchemaEdit(
    #[props(into)]
    id: String,
) -> Element {
    let id = use_memo(use_reactive!(|id| id));

    let message_box_task = use_coroutine_handle::<MessageBoxAction>();
    let api_task = use_coroutine_handle::<ApiRequestAction>();
    let auth_state = use_auth_state();

    page_init!("menu-schemas", PERMISSION_SCHEMAS_READ, auth_state);

    let future =
        use_resource(move || async move {
            request_fetch_task(url!(API_SCHEMA, &id())).await
        });

    let response = future.suspend()?;
    if response().is_null() { fail!(future) }

    let submit = move |event: Event<FormData>| {
        api_task.send(ApiRequestAction::PostThenBack(
            url!(API_SCHEMA),
            Some(json!({
                "id": event.get_str("id"),
                "slug": event.get_str("slug"),
                "title": event.get_str("title"),
                "kind": event.get_str("kind"),
                "permission": event.get_str("permission"),
                "fields": event.get_fields_array()
            })),
        ))
    };

    let delete = move |event: MouseEvent| {
        api_task.send(ApiRequestAction::DeleteThenBack(
            url!(API_SCHEMA, &id()),
            None,
        ))
    };

    rsx! {
        section {
            class: "flex grow flex-col select-none flex-row px-3 pr-20 sm:pr-16",
            form {
                class: "flex grow flex-col items-center gap-3",
                id: "schema-edit-form",
                autocomplete: "off",
                onsubmit: submit,

                input {
                    r#type: "hidden",
                    name: "id",
                    initial_value: response().get_string("id")
                }
                div {
                    class: "grid w-full grid-cols-1 sm:grid-cols-2 gap-5",
                    FormSchemaKindField {
                        init_kind: response().get_schema_kind(),
                        disabled: !id().eq(ID_CREATE)
                    }
                    FormPermissionsField {
                        init_permission: response().get_str("permission")
                        .unwrap_or(PERMISSION_PUBLIC.into()),
                        permissions: response()
                        .get_str_array("permissions")
                        .unwrap_or(vec![Cow::Borrowed(PERMISSION_PUBLIC)])
                    }
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
            }
            FormFieldsField {
                items: response().get_schema_fields().unwrap_or_default()
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
                form: "schema-edit-form",
                permission: PERMISSION_SCHEMAS_WRITE,
            }
        } else {
            EditorActions {
                form: "schema-edit-form",
                delete_event: delete,
                permission: PERMISSION_SCHEMAS_WRITE,
            }
        }
    }
}