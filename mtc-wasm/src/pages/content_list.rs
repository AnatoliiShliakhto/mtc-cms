use super::*;

#[component]
pub fn ContentList(
    #[props(into)]
    schema: String,
) -> Element {
    let schema = use_memo(use_reactive!(|schema| schema));

    let is_writer = use_auth_state()().has_role(ROLE_WRITER);
    let menu_item = format!("menu-{}", schema());
    breadcrumbs!(&menu_item);

    let future = value_future!(url!(API_CONTENTS, &schema()));
    let response = future.suspend()?;
    check_response!(response, future);

    rsx! {
        section {
            class: if is_writer {
                "w-full flex-grow xl:pr-16"
            } else {
                "w-full max-w-full flex flex-wrap grow mt-3 px-4 sm:px-0 ck-content justify-center"
            },
            h3 {
                class: "flex w-full flex-wrap pb-4 sm:pb-6",
                class: "justify-center text-2xl font-semibold text-center",
                { response().key_string("title") }
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
                    for item in response().key_obj::<Vec<Entry>>("entries")
                    .unwrap_or_default().iter() {{
                        let published = item.variant.clone().unwrap_or_default()
                        .as_bool().unwrap_or(false);
                        let slug = item.slug.to_owned();

                        rsx! {
                            tr {
                                onclick: move |_| {
                                    navigator()
                                    .push(route!(API_CONTENT, &schema(), &slug));
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