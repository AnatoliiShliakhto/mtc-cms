use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::pagination_model::PaginationModel;

pub fn PaginatorCompact(
    mut page: Signal<usize>,
    pagination: PaginationModel,
) -> Element {
    let i18 = use_i18();
    
    if pagination.total <= pagination.per_page { return rsx! {} }

    rsx! {
        div { class: "join",
            button { class: if pagination.has_previous_page { "join-item btn" } else { "join-item btn btn-disabled" },
                onclick: move |_| page.set(pagination.previous_page_number),
                "«"
            }
            button { class: "join-item btn",
                onclick: move |_| page.set(pagination.current_page),
                span {
                    { translate!(i18, "messages.page") }
                    " "
                    { pagination.current_page.to_string() }
                }
            }
            button { class: if pagination.has_next_page { "join-item btn" } else { "join-item btn btn-disabled" },
                onclick: move |_| page.set(pagination.next_page_number),
                "»"
            }
        }
    }
}