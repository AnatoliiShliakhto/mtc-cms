use super::*;

/// A component to render a search box.
#[component]
pub fn SearchBox() -> Element {
    let search_results = state_fn!(search_engine).result;

    rsx! {
        section {
            class: "prose-base w-full max-w-full flex flex-wrap grow mt-3 px-4 sm:px-0 \
            ck-content justify-center",
            article {
                class: "flex grow flex-col max-w-full lg:max-w-4xl overflow-x-auto",
                div {
                    class: "flex grow gap-5 justify-center items-center",
                    div {
                        class: "w-full divider text-xl",
                        { t!("caption-search-results") }
                        button {
                            class: "btn btn-sm btn-circle btn-ghost text-neutral hover:text-error",
                            onclick: move |_| state_fn!(search_engine_clear),
                            Icon { icon: Icons::Close, class: "size-6" }
                        }
                    }
                }
                ViewLinksSearch { results: search_results() }
            }
        }
    }
}