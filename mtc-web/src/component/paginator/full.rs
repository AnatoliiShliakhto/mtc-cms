use dioxus::prelude::*;

use mtc_model::pagination_model::PaginationModel;

pub fn PaginatorFull(
    mut page: Signal<usize>,
    pagination: Signal<PaginationModel>,
) -> Element {
    let total_pages = use_memo(move || pagination().total / pagination().per_page + 1 +
        match pagination().total % pagination().per_page {
            0 => 0,
            _ => 1
        }
    );

    if total_pages() <= 2 { return rsx! {}; }

    rsx! {
        div { class: "flex flex-wrap join",
            for i in 1..total_pages() {
                button { class: if i == pagination().current_page { "join-item btn btn-disabled btn-sm" } else { "join-item btn btn-sm" },
                    prevent_default: "onclick",
                    onclick: move |_| { page.set(i) },
                    { i.to_string() }
                }
            }
        }
    }
}