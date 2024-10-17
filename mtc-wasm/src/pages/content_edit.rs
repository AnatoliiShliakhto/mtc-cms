use super::*;

#[component]
pub fn ContentEdit(
    #[props(into)]
    schema: String,
    #[props(into)]
    slug: String,
) -> Element {
    let schema = use_memo(use_reactive!(|schema| schema));
    let slug = use_memo(use_reactive!(|slug| slug));

    breadcrumbs!("menu-content-edit");
    check_role!(ROLE_WRITER);

    let future = value_future!(url!(API_CONTENT, &schema(), &slug()));
    let response = future.suspend()?;
    check_response!(response, future);

    let id = response().key_string("id").unwrap_or_default();
    let content = response().key_obj::<Value>("data").unwrap_or_default();
    let fields = response().key_obj::<Vec<Field>>("fields").unwrap_or_default();
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

        let payload = json!({
            "id": event.get_str("id"),
            "slug": event.get_str("slug"),
            "title": event.get_str("title"),
            "published": event.get_bool("published"),
            "data": data
        });

        spawn(async move {
            if post_request!(url!(API_CONTENT, &schema(), &slug()), payload) {
                navigator().go_back();
            }
        });
    };

    let delete = move |event: MouseEvent| {
        spawn(async move {
            if delete_request!(url!(API_CONTENT, &schema(), &slug())) {
                navigator().replace(Route::Home {});
            }
        });
    };

    SessionStorage::set("contentId", &id)
        .map_err(|e| error!("{e:#?}"))
        .ok();

    rsx! {
        section {
            class: "flex grow select-none flex-row gap-6 px-3 pr-20 xl:pr-16",
            form {
                class: "flex grow flex-col items-center gap-3",
                id: "content-edit-form",
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
                for field in fields.iter() {{
                    match field.kind {
                        FieldKind::Str => rsx! {
                            FormTextField {
                                name: &field.slug,
                                title: &field.title,
                                initial_value: content.key_string(&field.slug)
                            }
                        },
                        FieldKind::Text => rsx! {
                            FormTextAreaField {
                                name: &field.slug,
                                title: &field.title,
                                initial_value: content.key_string(&field.slug)
                            }
                        },
                        FieldKind::Html => rsx! {
                            FormHtmlField {
                                name: &field.slug,
                                title: &field.title,
                                initial_value: content.key_string(&field.slug)
                            }
                        },
                        FieldKind::Links => {
                            let links_arr = content.key_obj::<Vec<LinkEntry>>(&field.slug)
                            .unwrap_or_default();
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
                                initial_value: content.key_obj::<Value>(&field.slug)
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
            created_by: response().key_string("created_by"),
            created_at: response().key_datetime("created_at"),
            updated_by: response().key_string("updated_by"),
            updated_at: response().key_datetime("updated_at"),
        }
        if schema().eq("page") | schema().eq("course") {
            EditorActions {
                form: "content-edit-form",
                permission: PERMISSION_PUBLIC_WRITE,
            }
        } else {
            EditorActions {
                form: "content-edit-form",
                delete_handler: delete,
                permission: PERMISSION_PUBLIC_WRITE,
            }
        }
        PublishedAction {
            checked: response().key_bool("published").unwrap_or_default()
        }
        StorageActions { id }
    }
}