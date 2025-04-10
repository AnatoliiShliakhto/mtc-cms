use super::*;

/// Header component with navigation bar, search box and theme switcher.
#[component]
pub fn Header() -> Element {
    let mut search_pattern = state_fn!(search_engine).pattern;

    rsx! {
        div { 
            class: "bg-base-100 text-base-content sticky top-0 z-[30] flex h-12 w-full",
            class: "justify-center bg-opacity-90 backdrop-blur-sm transition-shadow duration-100",
            class: "[transform:translate3d(0,0,0)] qr-element",
            nav {
                class: "navbar w-full p-0 px-1 min-h-12 h-12",
                div { 
                    class: "inline-flex flex-nowrap flex-1 items-center md:gap-1 lg:gap-2",
                    label {
                        class: "btn btn-ghost lg:hidden",
                        tabindex: "0",
                        r#for: "main-menu",
                        Icon { icon: Icons::Menu, class: "size-8 sm:size-6" }
                    }
                    form {
                        class: "flex-1 px-2",
                        autocomplete: "off",
                        onsubmit: move |event| {
                            let pattern = event.get_str("pattern").unwrap_or_default().to_string();
                            if pattern.is_empty() { return }
                            state_fn!(search_engine_clear);
                            navigator().push(route!(API_SEARCH, pattern));
                        },
                        label {
                            class: "w-full input input-sm items-center gap-2",
                            input {
                                class: "grow peer",
                                style: "max-width: inherit; width: 100%",
                                r#type: "search",
                                name: "pattern",
                                placeholder: &*t!("message-search"),
                                value: &*search_pattern(),
                                oninput: move |event| search_pattern.set(event.value().into()),
                            }
                            button {
                                class: "relative -right-3 btn btn-sm btn-ghost",
                                class: "opacity-30 peer-focus:opacity-100",
                                class: "peer-focus:text-accent",
                                Icon { icon: Icons::Search, class: "size-6" }
                            }
                        }
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