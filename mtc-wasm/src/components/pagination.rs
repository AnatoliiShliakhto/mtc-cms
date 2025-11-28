use super::*;

const PAGE_SIZE_SYMBOL: &str = "# ";
const PAGE_INDEX_SYMBOL: &str = "№ ";

#[component]
pub fn PaginationBar(
    page_size_signal: Signal<usize>,
    page_index_signal: Signal<usize>,
    number_of_pages_signal: Signal<usize>,
) -> Element {
    rsx! {
        div { class: "flex items-center justify-center",
            div { class: "flex items-center join",
                button {
                    class: "join-item btn",
                    disabled: page_index_signal() == 0,
                    onclick: move |event| {
                        event.prevent_default();
                        event.stop_propagation();

                        page_index_signal.set(page_index_signal() - 1);
                    },
                    {"|<"}
                }
                select {
                    class: "select join-item w-30",
                    onchange: move |event| {
                        event.stop_propagation();
                        page_index_signal.set(event.value().replace(PAGE_INDEX_SYMBOL, "").parse::<usize>().unwrap_or_default() - 1);
                    },
                    for page_index in 0..number_of_pages_signal() {
                        option {
                            selected: page_index == page_index_signal(),
                            {format!("{} {}", PAGE_INDEX_SYMBOL,  page_index + 1)}
                        }
                    }
                }
                select {
                    class: "select join-item w-30",
                    onchange: move |event| {
                        event.stop_propagation();
                        page_index_signal.set(0);
                        page_size_signal.set(event.value().replace(PAGE_SIZE_SYMBOL, "").parse::<usize>().unwrap_or_default());
                    },
                    for page_size_index in 1..=5 {
                        option { selected: page_size_signal() == page_size_index * 20,
                            {format!("{} {}", PAGE_SIZE_SYMBOL, page_size_index * 20)}
                        }
                    }
                }
                button {
                    class: "join-item btn",
                    disabled: page_index_signal() + 1 >= number_of_pages_signal(),
                        onclick: move |event| {
                        event.prevent_default();
                        event.stop_propagation();

                        page_index_signal.set(page_index_signal() + 1);
                    },
                    {">|"}
                }
            }
        }
    }
}
