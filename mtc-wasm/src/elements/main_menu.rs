use super::*;

pub fn MainMenu() -> Element {
    let mut menu_state = use_menu_state();
    
    rsx! {
        aside { 
            class: "flex flex-col bg-base-200 min-h-screen w-full sm:w-80",
            div { 
                class: "bg-base-200 sticky top-0 z-20 flex items-center justify-center \
                gap-2 bg-opacity-90 min-h-12 h-12 backdrop-blur",
                div { 
                    class: "inline-flex w-full flex-nowrap",
                    button { 
                        class: "grow btn btn-ghost gap-3 pl-12 lg:pl-0 rounded-none",
                        onclick: move |_| {
                            menu_state.set(false);
                            use_search_engine_drop();
                            navigator().push(Route::Home {});
                        },
                        img { 
                            class: "size-8 sm:size-6",
                            src: "/assets/logo.png" 
                        }
                        span { 
                            class: "flex flex-nowrap text-3xl sm:text-xl gap-3 items-center font-semibold",
                            { t!("site-short-title") }
                        }
                    }
                    button { 
                        class: "btn btn-ghost inline-flex lg:hidden rounded-none",
                        onclick: move |_| menu_state.set(false),
                        Icon { icon: Icons::Close, class: "size-6 mr-4 sm:mr-0" }
                    }
                }
            }
            ul { class: "menu menu-lg sm:menu-md",
                SideMenu {}
            }
            div { class: "flex flex-col grow justify-end",
                Footer {}
            }
        }
    }
}