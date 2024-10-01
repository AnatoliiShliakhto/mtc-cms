use super::*;

pub fn Schemas() -> Element {
    let auth_state = use_auth_state();
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();
    page_init!("menu-schemas", PERMISSION_SCHEMAS_READ, auth_state);

    let future =
        use_resource(move || async move {
            request_fetch_entries_task(url!("schemas")).await
        });
    let response = future.suspend()?;

    if response().is_none() { fail!(future) }

    rsx! {
        section {
            class: "w-full flex-grow sm:pr-16",
            table {
                class: "entry-table",
                thead {
                    tr {
                        th { class: "w-8" }
                        th {
                            class: "w-3/12",
                            { t!("field-slug") }
                        }
                        th {
                            class: "w-full",
                            { t!("field-title") }
                        }
                        th {
                            class: "w-36",
                            { t!("field-permission") }
                        }
                    }
                }
                tbody {
                    for item in response().unwrap_or(vec![]).iter() {{
                        let id = item.id.to_owned();
                        let details = serde_json::from_value::<SchemaEntryDetails>(item
                        .variant
                        .to_owned()
                        .unwrap_or_default())
                        .unwrap_or_default();

                        rsx! {
                            tr {
                                onclick: move |_| {
                                    navigator()
                                    .push(Route::SchemaEdit { id: id.to_string() });
                                },
                                td {
                                    match details.kind {
                                        SchemaKind::System => rsx!{
                                            Icon { icon: Icons::Lock, class: "size-4 text-error" }
                                        },
                                        SchemaKind::User => rsx!{
                                            Icon { icon: Icons::People, class: "size-4 text-error" }
                                        },
                                        SchemaKind::Page => rsx!{
                                            Icon { icon: Icons::Description, class: "size-4" }
                                        },
                                        SchemaKind::Pages => rsx!{
                                            Icon { icon: Icons::Folder, class: "size-4" }
                                        },
                                        SchemaKind::Links => rsx!{
                                            Icon { icon: Icons::Description, class: "size-4" }
                                        },
                                        SchemaKind::Course => rsx!{
                                            Icon { icon: Icons::Diagram3, class: "size-4 text-accent" }
                                        },
                                        SchemaKind::Quiz => rsx!{},
                                    }
                                }
                                td {
                                    { item.slug.as_ref() }
                                }
                                td {
                                    { item.title.as_ref() }
                                }
                                td {
                                    { details.permission }
                                }
                            }
                        }
                    }}
                }
                EntriesActions {
                    future,
                    route: Route::SchemaEdit { id: ID_CREATE.to_string() }.to_string(),
                    permission: PERMISSION_SCHEMAS_WRITE,
                }
            }
        }
    }
}