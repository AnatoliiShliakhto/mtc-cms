use super::*;

pub fn Layout() -> Element
{
    use_coroutine(api_request_service);

    let mut menu_state = use_init_menu_state();

    let message_box = use_message_box();
    let search_result_empty = use_memo(|| use_search_engine()().is_empty());

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
                            SearchResult {}
                        }
                    }
                }
            }
            div { 
                class: "drawer-side z-[40]",
                label { 
                    class: "drawer-overlay", 
                    r#for: "main-menu" 
                }
                MainMenu {}
            }
        }
        if let Some((kind, message, task, task_args)) = message_box() {
            MessageBox {
                kind,
                message,
                task,
                task_args,
            }            
        }
        button {
            id: "scrollUpButton",
            class: "fixed btn btn-circle btn-neutral opacity-60 hover:opacity-100 \
            right-4 bottom-4 hidden",
            "onclick": "window.scrollTo(0, 0);",
            Icon { icon: Icons::ArrowUp, class: "size-8" }
        }
        script { { EVAL_SCROLL_UP } }
    }
}