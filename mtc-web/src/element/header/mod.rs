use dioxus::prelude::*;

use crate::component::account_controller::AccountControllerComponent;
use crate::component::language_switcher::LanguageSwitcherComponent;
use crate::component::theme_switcher::ThemeSwitcherComponent;

pub fn Header() -> Element {
    rsx! {
        div { class: "bg-base-100 text-base-content sticky top-0 z-[30] flex h-12 w-full justify-center bg-opacity-90 backdrop-blur transition-shadow duration-100 [transform:translate3d(0,0,0)]", 
            nav { class: "navbar w-full p-0 min-h-12",
                div { class: "flex flex-1 md:gap-1 lg:gap-2",
                    label { 
                        tabindex: "0", 
                        class: "btn btn-ghost lg:hidden",
                        r#for: "main-menu",
                        svg {
                            "fill": "none",
                            "viewBox": "0 0 24 24",
                            "xmlns": "http://www.w3.org/2000/svg",
                            "stroke": "currentColor",
                            class: "h-5 w-5",
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                "stroke-width": "2",
                                "d": "M4 6h16M4 12h8m-8 6h16"
                            }
                        }
                    }
                    div { class: "flex w-full lg:hidden justify-center",
                        Link { class: "btn btn-ghost text-xl flex lg:hidden", to: crate::router::Route::HomePage {}, "MTC-CMS" }
                    }
                    span { class: "hidden lg:flex px-5 text-xl", style:"font-weight: 600;", "military training center" }
                }
                div { class: "flex-0",
                    div { class: "join",
                        LanguageSwitcherComponent {}
                        ThemeSwitcherComponent {}
                        AccountControllerComponent {}
                    }
                }
            }    
        }
    }
}