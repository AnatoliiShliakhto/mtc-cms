use super::*;

#[component]
pub fn Search(
    #[props(into)]
    pattern: ReadOnlySignal<String>,
) -> Element {
    let payload = Value::String(pattern());

    breadcrumbs!("menu-search");

    let future = value_future!(url!(API_SEARCH), payload);
    let response = future.suspend()?;
    check_response!(response, future);

    let results = response()
        .self_obj::<Vec<SearchIdxDto>>()
        .unwrap_or_default();

    rsx! {
        section {
            class: "prose prose-base w-full max-w-full flex flex-wrap grow mt-3 px-4 sm:px-0 \
            ck-content justify-center",
            article {
                class: "flex grow flex-col max-w-full lg:max-w-4xl overflow-x-auto",
                div {
                    class: "flex grow gap-5 justify-center items-center",
                    div {
                        class: "text-xl",
                        { t!("caption-search-results") }
                        ": '"
                        { pattern() }
                        "'"
                    }
                }
                if results.is_empty() {
                    h4 {
                        class: "flex w-full justify-center",
                        { t!("message-no-results") }
                    }
                } else {
                    dl {
                        class: "file-stack",
                        for item in results.iter() {
                            dd {
                                class: "group",
                                match item.kind {
                                    SearchKind::LocalLink => rsx! {
                                        Icon {
                                            icon: Icons::Description,
                                            class: "size-4 text-primary group-hover:animate-ping"
                                        }
                                    },
                                    SearchKind::Link => rsx! {
                                        Icon {
                                            icon: Icons::Link,
                                            class: "size-4 text-primary group-hover:animate-ping"
                                        }
                                    },
                                    SearchKind::File => rsx! {
                                        Icon {
                                            icon: Icons::File,
                                            class: "size-4 group-hover:animate-ping"
                                        }
                                    },
                                    SearchKind::FileWord => rsx! {
                                        Icon {
                                            icon: Icons::FileWord,
                                            class: "size-4 group-hover:animate-ping"
                                        }
                                    },
                                    SearchKind::FileExcel => rsx! {
                                        Icon {
                                            icon: Icons::FileExcel,
                                            class: "size-4 group-hover:animate-ping"
                                        }
                                    },
                                    SearchKind::FilePowerPoint => rsx! {
                                        Icon {
                                            icon: Icons::FilePowerPoint,
                                            class: "size-4 group-hover:animate-ping"
                                        }
                                    },
                                    SearchKind::FilePdf => rsx! {
                                        Icon {
                                            icon: Icons::FilePdf,
                                            class: "size-4 group-hover:animate-ping"
                                        }
                                    },
                                    SearchKind::Course => rsx! {
                                        Icon {
                                            icon: Icons::Diagram3,
                                            class: "size-4 text-info group-hover:animate-ping"
                                        }
                                    },
                                }
                                if item.kind == SearchKind::LocalLink
                                || item.kind == SearchKind::Course {
                                    Link {
                                        //class: "group-hover:animate-pulse",
                                        onclick: move |_| use_search_engine_drop(),
                                        to: &*item.url,
                                        { item.title.to_owned() }
                                    }
                                } else {
                                    a {
                                        //class: "group-hover:animate-pulse",
                                        target: "_blank",
                                        href: &*item.url,
                                        { item.title.to_owned() }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}