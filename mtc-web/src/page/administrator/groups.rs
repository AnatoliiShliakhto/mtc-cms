use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::component::loading_box::LoadingBoxComponent;
use crate::component::reloading_box::ReloadingBoxComponent;
use crate::global_signal::APP;
use crate::handler::group_handler::GroupHandler;

#[component]
pub fn Groups() -> Element {
    let i18 = use_i18();

    let mut page = use_signal(|| 1usize);

    let mut future = use_resource(move || async move { APP.read().get_group_list(page()).await });

    match &*future.read_unchecked() {
        Some(Ok(response)) => {
            let data = response.data.clone();
            let pagination = response.pagination.clone().unwrap();

            rsx! {
                div { class: "flex flex-col gap-3 overflow-x-auto",
                    table { class: "table",
                        thead {
                            tr {
                                th { "slug" }
                                th { "title" }
                                th { "created at" }
                                th { "updated at" }
                            }
                        }
                        tbody {
                            for group in data {
                                tr {
                                    th { { group.slug } }
                                    td { { group.title } }
                                    td { { group.created_at.format("%H:%M %d/%m/%Y").to_string() } }
                                    td { { group.updated_at.format("%H:%M %d/%m/%Y").to_string() } }
                                }
                            }
                        }
                    }
                    div { class: "join self-center",
                        button { prevent_default: "onclick",
                            class: "join-item btn",
                            onclick: move |_| { page.set(pagination.previous_page_number) },
                            "«"
                        }
                        button { prevent_default: "onclick",
                            class: "join-item btn",
                            onclick: move |_| { future.restart() },
                            span { "page " { pagination.current_page.to_string() } }
                        }
                        button { prevent_default: "onclick",
                            class: "join-item btn",
                            onclick: move |_| { page.set(pagination.next_page_number) },
                            "»"
                        }
                    }
                }
            }
        }
        Some(Err(e)) => rsx! { ReloadingBoxComponent { message: e.message(), future: future } },
        None => rsx! { LoadingBoxComponent {} },
    }
}