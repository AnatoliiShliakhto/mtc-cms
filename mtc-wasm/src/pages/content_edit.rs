use super::*;

#[component]
pub fn ContentEdit(
    #[props(into)]
    schema: String,
    #[props(into)]
    slug: String,
) -> Element {
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();
    let api_task = use_coroutine_handle::<ApiRequestAction>();
    let auth_state = use_auth_state();
    let api_client = use_api_client();

    page_init!("menu-content-edit", PERMISSION_PUBLIC_WRITE, auth_state);

    let schema = use_memo(use_reactive!(|schema| schema));
    let slug = use_memo(use_reactive!(|slug| slug));

    let future =
        use_resource(move || async move {
            request_fetch_task(url!(API_CONTENT, &schema(), &slug())).await
        });

    let response = future.suspend()?;
    if response().is_null() { fail!(future) }

    let id = response().get_string("id").unwrap_or_default();
    let content = response().get_object::<Value>("data").unwrap_or_default();
    let fields = response().get_schema_fields().unwrap_or_default();
    let fields_cloned = fields.clone();

    let submit = move |event: Event<FormData>| {
        let mut data = json!({});

        for field in fields_cloned.iter() {
            match field.kind {
                FieldKind::Str
                | FieldKind::Text
                | FieldKind::Html => {
                    data.insert_value(
                        &field.slug,
                        event.get_str(&field.slug).into()
                    );
                }
                FieldKind::Links => {
                    data.insert_value(
                        &field.slug,
                        json!(event.get_links_array(&field.slug))
                    );
                }
                FieldKind::Course => {
                    data.insert_value(
                        &field.slug,
                        serde_json::from_str(&event.get_str(&field.slug).unwrap_or_default())
                            .unwrap_or_default()
                    );
                }
                FieldKind::Decimal => {}
                FieldKind::DateTime => {}
            }
        }

        let url: String = url!(API_CONTENT, &schema(), &slug());
        let json_obj = json!({
            "id": event.get_str("id"),
            "slug": event.get_str("slug"),
            "title": event.get_str("title"),
            "published": event.get_bool("published"),
            "data": data
        });

        //todo research MutBorrow bug bit later
        spawn(async move {
            match api_client()
                .post(&*url)
                .json(&json_obj)
                .send()
                .await
                .consume()
                .await {
                Ok(_) =>
                    if navigator().can_go_back() {
                        message_box_task.send(MessageBoxAction::Clear);
                        navigator().go_back()
                    } else {
                        message_box_task
                            .send(MessageBoxAction::Success(t!("message-success-post")))
                    }
                Err(e) =>
                    message_box_task.send(MessageBoxAction::Error(e.message())),
            }
        });

        /*
        api_task.send(ApiRequestAction::PostThenBack(
            url!(API_CONTENT, &schema(), &slug()),
            Some(json!({
                "id": event.get_str("id"),
                "slug": event.get_str("slug"),
                "title": event.get_str("title"),
                "published": event.get_bool("published"),
                "data": data
            })),
        ))
        */
    };

    let delete = move |event: MouseEvent| {
        api_task.send(ApiRequestAction::DeleteThenBack(
            url!(API_CONTENT, &schema(), &slug()),
            None,
        ))
    };

    SessionStorage::set("contentId", &id)
        .map_err(|e| error!("{e:#?}"))
        .ok();

    rsx! {
        section {
            class: "flex grow select-none flex-row gap-6 px-3 pr-20 sm:pr-16",
            form {
                class: "flex grow flex-col items-center gap-3",
                id: "content-edit-form",
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
                for field in fields.iter() {{
                    match field.kind {
                        FieldKind::Str => rsx! {
                            FormTextField {
                                name: &field.slug,
                                title: &field.title,
                                initial_value: content.get_string(&field.slug)
                            }
                        },
                        FieldKind::Text => rsx! {
                            FormTextAreaField {
                                name: &field.slug,
                                title: &field.title,
                                initial_value: content.get_string(&field.slug)
                            }
                        },
                        FieldKind::Html => rsx! {
                            FormHtmlField {
                                name: &field.slug,
                                title: &field.title,
                                initial_value: content.get_string(&field.slug)
                            }
                        },
                        FieldKind::Links => {
                            let links_arr =
                            content.get_object::<Vec<LinkEntry>>(&field.slug).unwrap_or(vec![]);
                            let mut links_strings: Vec<String> = vec![];
                            for link in links_arr {
                                links_strings
                                .push(format!("{}; {}", link.title.trim(), link.url.trim()));
                            }

                            rsx!{
                                FormTextAreaField {
                                    name: &field.slug,
                                    title: &field.title,
                                    initial_value: links_strings.join("\r\n")
                                }
                            }
                        },
                        FieldKind::Course => rsx! {
                            input {
                                r#type: "hidden",
                                name: field.slug.as_ref(),
                                initial_value: content.get_object::<Value>(&field.slug)
                                .unwrap_or_default().to_string()
                            }
                        },
                        FieldKind::Decimal => rsx!{},
                        FieldKind::DateTime => rsx!{},
                    }
                }}
            }
        }
        EntryInfoBox {
            created_by: response().get_string("created_by"),
            created_at: response().get_datetime("created_at"),
            updated_by: response().get_string("updated_by"),
            updated_at: response().get_datetime("updated_at"),
        }
        if schema().eq("page") | schema().eq("course") {
            EditorActions {
                form: "content-edit-form",
                permission: PERMISSION_PUBLIC_WRITE,
            }
        } else {
            EditorActions {
                form: "content-edit-form",
                delete_event: delete,
                permission: PERMISSION_GROUPS_WRITE,
            }
        }
        PublishedAction {
            checked: response().get_bool("published").unwrap_or_default()
        }
        StorageActions { id }
    }
}