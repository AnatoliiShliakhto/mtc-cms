use super::*;

/// The main menu of the WASM application.
#[component]
pub fn MainMenu() -> Element {

    rsx! {
        aside {
            class: "flex flex-col bg-base-200 min-h-screen w-full sm:w-80",
            div { 
                class: "bg-base-200 sticky top-0 z-20 flex items-center justify-center",
                class: "gap-2 bg-opacity-90 min-h-12 h-12 backdrop-blur",
                div { 
                    class: "inline-flex w-full flex-nowrap",
                    button { 
                        class: "grow btn btn-ghost gap-3 pl-12 lg:pl-0 rounded-none",
                        onclick: move |_| {
                            state!(set_menu, false);
                            state_fn!(search_engine_clear);
                            navigator().push(route!());
                        },
                        img { 
                            class: "size-8 sm:size-6",
                            src: "/assets/logo.webp"
                        }
                        span { 
                            class: "flex flex-nowrap text-3xl sm:text-xl gap-3",
                            class: "items-center font-semibold",
                            { t!("site-short-title") }
                        }
                    }
                    button { 
                        class: "btn btn-ghost inline-flex lg:hidden rounded-none",
                        onclick: move |_| state!(set_menu, false),
                        Icon { icon: Icons::Close, class: "size-6" }
                    }
                }
            }
            ul { class: "menu menu-lg sm:menu-md main-menu",
                SideMenu {}
            }
            div { class: "flex flex-col grow justify-end",
                Footer {}
            }
        }
    }
}