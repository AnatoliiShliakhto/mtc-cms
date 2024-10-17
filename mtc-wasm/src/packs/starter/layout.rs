use super::*;

#[component]
pub fn Layout() -> Element
{
    let mut menu_state = use_init_menu_state();
    let search_result_empty = use_memo(move || use_search_engine()().is_empty());

    rsx! {
        div {
            class: "bg-base-100 drawer lg:drawer-open",
            input {
                class: "drawer-toggle",
                id: "main-menu",
                r#type: "checkbox",
                checked: menu_state(),
                onchange: move |event| menu_state.set(event.checked())
            }
            div {
                class: "drawer-side z-[40]",
                label {
                    class: "drawer-overlay",
                    r#for: "main-menu"
                }
                MainMenu {}
            }
            div {
                class: "drawer-content",
                Header {}
                Breadcrumbs {}
                div {
                    class: "max-w-[100vw] pt-3 pb-16 sm:px-6 break-words overflow-x-auto",
                    SuspenseBoundary {
                        fallback: |context: SuspenseContext| rsx! {
                            Loading {}
                        },
                        if search_result_empty() {
                            Outlet::<Route> {}
                        } else {
                            SearchBox {}
                        }
                    }
                }
            }
        }
    }
}