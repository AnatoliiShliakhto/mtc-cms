use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::pagination_model::PaginationModel;

pub fn PaginatorCompact(
    mut page: Signal<usize>,
    pagination: PaginationModel,
) -> Element {
    let i18 = use_i18();

    rsx! {
        div { class: "join",
            button { class: "join-item btn",
                prevent_default: "onclick",
                onclick: move |_| { page.set(pagination.previous_page_number) },
                "«"
            }
            button { class: "join-item btn",
                prevent_default: "onclick",
                onclick: move |_| { page.set(pagination.current_page) },
                span {
                    { translate!(i18, "messages.page") }
                    " "
                    { pagination.current_page.to_string() }
                }
            }
            button { class: "join-item btn",
                prevent_default: "onclick",
                onclick: move |_| { page.set(pagination.next_page_number) },
                "»"
            }
        }
    }
}