use super::*;

pub fn Header() -> Element {
    let mut search_pattern = use_search_engine_pattern();

    rsx! {
        div { 
            class: "bg-base-100 text-base-content sticky top-0 z-[30] flex h-12 w-full \
            justify-center bg-opacity-90 backdrop-blur transition-shadow duration-100 \
            [transform:translate3d(0,0,0)]",
            nav { 
                class: "navbar w-full p-0 min-h-12 h-12",
                div { 
                    class: "inline-flex flex-nowrap flex-1 md:gap-1 lg:gap-2",
                    label {
                        class: "btn btn-ghost lg:hidden rounded-none",
                        tabindex: "0",
                        r#for: "main-menu",
                        Icon { icon: Icons::Menu, class: "size-8 sm:size-6" }
                    }
                    
                    label { 
                        class: "input input-bordered input-sm flex grow \
                        mx-2 sm:mx-4 items-center gap-2",
                        input { 
                            class: "grow",
                            style: "max-width: inherit; width: 100%",
                            r#type: "search",
                            placeholder: &*t!("message-search"),
                            value: &*search_pattern(),
                            oninput: move |event| search_pattern.set(event.value().into()),
                        }
                        Icon { icon: Icons::Search, class: "size-6 sm:size-4 opacity-70" }
                    }
                }
                
                div { 
                    class: "flex-0",
                    div { 
                        class: "join rounded-none",
                        //LanguageSwitcher {}
                        ThemeSwitcher {}
                        ProfileController {}
                    }
                }
            }
        }
    }
}