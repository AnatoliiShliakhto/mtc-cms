use super::*;

/// A component to view a content entry, given the schema and slug to identify the entry.
///
/// The component renders the content fields as a list of [`Prose`] elements, with the
/// field type determining the specific component used to render the field. The
/// [`ContentActions`] component is also rendered if the user has the writer role.
///
/// # Props
///
/// * `schema`: The schema to render the content entry for.
/// * `slug`: The slug of the content entry to render.
/// * `arg`: An optional argument to pass to the [`ViewCourseField`] component.
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

    let is_writer = state!(auth).has_role(ROLE_WRITER);
    let menu_item = if schema().eq("page") || schema().eq("course") {
        format!("menu-{}", slug())
    } else {
        format!("menu-{}", schema())
    };

    breadcrumbs!(&menu_item);

    let future = value_future!(url!(API_CONTENT, &schema(), &slug()));
    let response = future.suspend()?;
    check_response!(response, future);

    let content = response().key_obj::<Value>("data").unwrap_or_default();

    rsx! {
        section {
            class: "w-full max-w-full flex flex-wrap grow mt-3 px-4 sm:px-0",
            class: "ck-content justify-center overflow-x-auto",
            h3 {
                class: "flex w-full flex-wrap pb-4 sm:px-4",
                class: "justify-center text-2xl font-semibold text-center",
                { response().key_string("title") }
            }
            div {
                class: "prose prose-base flex grow flex-col max-w-full lg:max-w-4xl",
                for field in response().key_obj::<Vec<Field>>("fields")
                .unwrap_or_default().iter() {{
                    match field.kind {
                        FieldKind::Str => rsx! {
                            ViewStringField {
                                value: content.key_string(&field.slug)
                            }
                        },
                        FieldKind::Text => rsx! {
                            ViewTextField {
                                value: content.key_string(&field.slug)
                            }
                        },
                        FieldKind::Html => rsx! {
                            ViewHtmlField {
                                value: content.key_string(&field.slug)
                            }
                        },
                        FieldKind::Links => rsx! {
                            ViewLinksField {
                                value: content.key_obj::<Value>(&field.slug)
                            }
                        },
                        FieldKind::Course => rsx! {
                            ViewCourseField {
                                slug,
                                value: content.key_obj::<Value>(&field.slug),
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
