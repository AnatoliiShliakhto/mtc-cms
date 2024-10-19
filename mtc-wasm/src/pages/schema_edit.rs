use super::*;

#[component]
pub fn SchemaEdit(
    #[props(into)]
    id: String,
) -> Element {
    let id = use_memo(use_reactive!(|id| id));

    breadcrumbs!("menu-schemas");
    check_permission!(PERMISSION_SCHEMAS_READ);

    let future = value_future!(url!(API_SCHEMA, &id()));
    let response = future.suspend()?;
    check_response!(response, future);

    let submit = move |event: Event<FormData>| {
        let payload = json!({
            "id": event.get_str("id"),
            "slug": event.get_str("slug"),
            "title": event.get_str("title"),
            "kind": event.get_str("kind"),
            "permission": event.get_str("permission"),
            "fields": event.get_fields_array()
        });

        spawn(async move {
            if post_request!(url!(API_SCHEMA), payload) {
                navigator().replace(route!(API_ADMINISTRATOR, API_SCHEMAS));
            }
        });
    };

    let delete = move |event: MouseEvent| {
        spawn(async move {
            if delete_request!(url!(API_SCHEMA, &id())) {
                navigator().replace(route!(API_ADMINISTRATOR, API_SCHEMAS));
            }
        });
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
                    initial_value: response().key_string("id")
                }
                if id().ne(ID_CREATE) {
                    input {
                        r#type: "hidden",
                        name: "kind",
                        initial_value: response().key_obj::<SchemaKind>("kind")
                        .unwrap_or_default().to_string()
                    }
                }
                div {
                    class: "grid w-full grid-cols-1 sm:grid-cols-2 gap-5",
                    FormSchemaKindField {
                        init_kind: response().key_obj::<SchemaKind>("kind")
                        .unwrap_or_default(),
                        disabled: !id().eq(ID_CREATE)
                    }
                    FormPermissionsField {
                        init_permission: response().key_string("permission")
                        .unwrap_or(PERMISSION_PUBLIC.into()),
                        permissions: response().key_obj::<Vec<Cow<'static, str>>>("permissions")
                        .unwrap_or(vec![Cow::Borrowed(PERMISSION_PUBLIC)])
                    }
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
            FormFieldsField {
                items: response().key_obj::<Vec<Field>>("fields").unwrap_or_default()
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
                form: "schema-edit-form",
                permission: PERMISSION_SCHEMAS_WRITE,
            }
        } else {
            EditorActions {
                form: "schema-edit-form",
                delete_handler: delete,
                permission: PERMISSION_SCHEMAS_WRITE,
            }
        }
    }
}