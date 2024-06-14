use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::APP_STATE;
use crate::component::loading_box::LoadingBoxComponent;
use crate::component::paginator::{PaginatorComponent, PaginatorComponentMode};
use crate::component::reloading_box::ReloadingBoxComponent;
use crate::handler::group_handler::GroupHandler;

#[component]
pub fn Groups() -> Element {
    let app_state = APP_STATE.signal();
    let i18 = use_i18();

    let page = use_signal(|| 1usize);

    let future = use_resource(move || async move { app_state.peek().api.get_group_list(page()).await });

    match &*future.read_unchecked() {
        Some(Ok(response)) => {
            let data = response.data.clone();
            let pagination_compact = response.pagination.clone().unwrap();
            let pagination_full = response.pagination.clone().unwrap();

            rsx! {
                div { class: "flex flex-col gap-3 overflow-auto grow",
                    div { class: "flex flex-wrap",
                        PaginatorComponent { mode: PaginatorComponentMode::Compact, page: page, pagination: pagination_compact }
                    }
                    table { class: "table",
                        thead {
                            tr {
                                th { { translate!(i18, "messages.slug") } }
                                th { { translate!(i18, "messages.title") } }
                                th { { translate!(i18, "messages.created_at") } }
                                th { { translate!(i18, "messages.updated_at") } }
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
                    PaginatorComponent { mode: PaginatorComponentMode::Full, page: page, pagination: pagination_full }
                }
            }
        }
        Some(Err(e)) => rsx! { ReloadingBoxComponent { message: e.message(), resource: future } },
        None => rsx! { LoadingBoxComponent {} },
    }
}