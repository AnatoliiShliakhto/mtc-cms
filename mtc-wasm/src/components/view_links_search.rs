use super::*;

/// A component to display a list of search results as links with icons.
#[component]
pub fn ViewLinksSearch(
    results: Vec<SearchIdxDto>,
) -> Element {

    rsx! {
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
                                onclick: move |_| state_fn!(search_engine_clear),
                                to: &*item.url,
                                { &*item.title }
                            }
                        } else if item.kind == SearchKind::Link {
                            a {
                                href: &*item.url,
                                onclick: jsFfiHandleOpenLinkEvent,
                                { &*item.title }
                            }
                        } else {
                            a {
                                href: &*item.url,
                                onclick: jsFfiHandleOpenDownloadedLinkEvent,
                                { &*item.title }
                            }
                        }
                    }
                }
            }
        }
    }
}