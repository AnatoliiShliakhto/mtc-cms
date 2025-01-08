use super::*;

/// A root component of the application, it contains the main menu and the content area.
#[component]
pub fn Layout() -> Element
{
    let menu_state = state!(menu);
    let search_result_empty = use_memo(move ||
        (state_fn!(search_engine).result)().is_empty()
    );

    rsx! {
        div {
            class: "drawer lg:drawer-open",
            input {
                class: "drawer-toggle",
                id: "main-menu",
                r#type: "checkbox",
                checked: menu_state,
                onchange: move |event| use_state().set_menu(event.checked())
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
                    class: "flex-grow flex-col max-w-[100vw]",
                    class: "pt-3 pb-16 sm:px-6 break-words overflow-x-auto",
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