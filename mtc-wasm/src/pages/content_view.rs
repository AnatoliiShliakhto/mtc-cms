use super::*;

#[component]
pub fn ContentView(
    #[props(into)]
    schema: String,
    #[props(into)]
    slug: String,
    #[props]
    arg: Option<String>,
) -> Element {
    let schema = use_memo(use_reactive!(|schema| schema));
    let slug = use_memo(use_reactive!(|slug| slug));
    let arg = use_memo(use_reactive!(|arg| arg.unwrap_or_default()));

    let auth_state = use_auth_state();
    let is_writer = auth_state().has_role(ROLE_WRITER);
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();

    page_init!(&format!("menu-{}", schema()));

    let future =
        use_resource(use_reactive!(|(slug,)| async move {
            request_fetch_task(url!(API_CONTENT, &schema(), &slug())).await
        }));
    let response = future.suspend()?;
    if response().is_null() { fail!(future) }

    let content: Value = response().get_object("data").unwrap_or_default();

    rsx! {
        section {
            class: "w-full max-w-full flex flex-wrap grow mt-3 px-4 sm:px-0 \
                    ck-content justify-center overflow-x-auto",
            h3 {
                class: "flex w-full flex-wrap pb-4 sm:px-4 justify-center text-2xl font-semibold",
                { response().get_string("title") }
            }
            div {
                class: "prose prose-base flex grow flex-col max-w-full lg:max-w-4xl",
                for field in response().get_schema_fields().unwrap_or_default().iter() {{
                    match field.kind {
                        FieldKind::Str => rsx! {
                            ViewStringField {
                                value: content.get_string(&field.slug)
                            }
                        },
                        FieldKind::Text => rsx! {
                            ViewTextField {
                                value: content.get_string(&field.slug)
                            }
                        },
                        FieldKind::Html => rsx! {
                            ViewHtmlField {
                                value: content.get_string(&field.slug)
                            }
                        },
                        FieldKind::Links => rsx! {
                            ViewLinksField {
                                value: content.get_object::<Value>(&field.slug)
                            }
                        },
                        FieldKind::Course => rsx! {
                            ViewCourseField {
                                slug,
                                value: content.get_object::<Value>(&field.slug),
                                arg: arg()
                            }
                        },
                        FieldKind::Decimal => rsx!{},
                        FieldKind::DateTime => rsx!{},
                    }
                }}
            }
            if is_writer {
                ContentActions {
                    schema: schema(),
                    slug: slug()
                }
            }
        }
    }
}
