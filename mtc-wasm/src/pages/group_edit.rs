use super::*;

#[component]
pub fn GroupEdit(
    #[props]
    id: ReadOnlySignal<String>
) -> Element {
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();
    let api_task = use_coroutine_handle::<ApiRequestAction>();
    let auth_state = use_auth_state();
    page_init!("menu-groups", PERMISSION_GROUPS_READ, auth_state);

    let future =
        use_resource(move || async move {
            request_fetch_task(url!("group", &id())).await
        });

    let response = future.suspend()?;

    let submit = move |event: Event<FormData>| {
        api_task.send(ApiRequestAction::PostThenBack(
            url!("group"),
            Some(json!({
                "id": event.get_str("id"),
                "slug": event.get_str("slug"),
                "title": event.get_str("title")
            })),
        ))
    };

    let delete = move |event: MouseEvent| {
        api_task.send(ApiRequestAction::DeleteThenBack(
            url!("group",  &id()),
            None,
        ))
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
                form: "group-edit-form",
            }
        } else {
            EditorActions {
                form: "group-edit-form",
                delete_event: delete,
            }
        }
    }
}