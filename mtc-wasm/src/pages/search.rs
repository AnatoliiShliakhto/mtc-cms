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
                ViewLinksSearch { results }
            }
        }
    }
}