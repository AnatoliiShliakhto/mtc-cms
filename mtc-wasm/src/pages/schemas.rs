use super::*;

/// Component for showing and managing all schemas.
#[component]
pub fn Schemas() -> Element {
    breadcrumbs!("menu-schemas");
    check_permission!(PERMISSION_SCHEMAS_READ);

    let future = value_future!(url!(API_SCHEMAS));
    let response = future.suspend()?;
    check_response!(response, future);

    rsx! {
        section {
            class: "w-full flex-grow xl:pr-16",
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
                    for schema in response()
                    .self_obj::<Vec<Entry>>()
                    .unwrap_or_default().iter() {{
                        let id = schema.id.to_owned();
                        let details = schema.variant.clone()
                        .unwrap_or_default()
                        .self_obj::<SchemaEntryDetails>()
                        .unwrap_or_default();

                        rsx! {
                            tr {
                                onclick: move |_| {
                                    navigator()
                                    .push(route!(API_ADMINISTRATOR, API_SCHEMA, &id));
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
                                    { schema.slug.as_ref() }
                                }
                                td {
                                    { schema.title.as_ref() }
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
                    route: route!(API_ADMINISTRATOR, API_SCHEMA, ID_CREATE),
                    permission: PERMISSION_SCHEMAS_WRITE,
                }
            }
        }
    }
}