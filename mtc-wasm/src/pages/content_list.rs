use super::*;

#[component]
pub fn ContentList(
    #[props(into)]
    schema: String,
) -> Element {
    let schema = use_memo(use_reactive!(|schema| schema));

    let auth_state = use_auth_state();
    let is_writer = auth_state().has_role(ROLE_WRITER);
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();

    page_init!(&format!("menu-{}", schema()));

    let future =
        use_resource(move || async move {
            request_fetch_task(url!(API_CONTENTS, &schema())).await
        });
    let response = future.suspend()?;
    if response().is_null() { fail!(future) }

    rsx! {
        section {
            class: if is_writer {
                "w-full flex-grow sm:pr-16"
            } else {
                "w-full max-w-full flex flex-wrap grow mt-3 px-4 sm:px-0 ck-content justify-center"
            },
            h3 {
                class: "flex w-full flex-wrap pb-4 sm:pb-6 justify-center text-2xl font-semibold",
                { response().get_string("title") }
            }

            table {
                class: "entry-table",
                if is_writer {
                    thead {
                        tr {
                            th { class: "size-10" }
                            th { { t!("field-title") } }
                        }
                    }
                }
                tbody {
                    for item in response().get_entries("entries").unwrap_or(vec![]).iter() {{
                        let published = item.variant.clone().unwrap_or_default()
                        .as_bool().unwrap_or(false);
                        let slug = item.slug.to_owned();

                        rsx! {
                            tr {
                                onclick: move |_| {
                                    navigator()
                                    .push(Route::ContentView {
                                        schema: schema(),
                                        slug: slug.to_string()
                                    });
                                },
                                if is_writer {
                                    td {{
                                        if published {
                                            rsx! {
                                                Icon {
                                                    icon: Icons::Eye,
                                                    class: "size-4 text-success"
                                                }
                                            }
                                        } else {
                                            rsx! {
                                                Icon {
                                                    icon: Icons::EyeSlash,
                                                    class: "size-4 text-warning"
                                                }
                                            }
                                        }
                                    }}
                                }
                                td {
                                    { item.title.as_ref() }
                                }
                            }
                        }
                    }}
                }
                if is_writer && schema().ne("page") && schema().ne("course") {
                    ContentListActions {
                        future,
                        schema: schema()
                    }
                }
            }
        }
    }
}