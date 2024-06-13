use dioxus::prelude::*;

use crate::component::account_controller::AccountControllerComponent;
use crate::component::language_switcher::LanguageSwitcherComponent;
use crate::component::theme_switcher::ThemeSwitcherComponent;
use crate::element::header::logo::HeaderLogo;
use crate::router::Route::HomePage;

mod logo;

pub fn Header() -> Element {
    rsx! {
        div { class: "navbar sticky left-0 top-0 right-0 z-[1] bg-base-100 shadow-md p-0 min-w-72 min-h-12 border-input-bordered",
            div { class: "navbar-start",
                HeaderLogo {}
                div { class: "dropdown",
                    div { role: "button", tabindex: "0", class: "btn btn-ghost md:hidden",
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
                    ul { class: "menu menu-sm dropdown-content z-[1] p-2 shadow bg-base-100",
                        li { class: "rounded-md",
                            Link { to: HomePage {}, "Головна" } }
                        li {
                            details {
                                summary { { "Інструктору" } }
                                ul { class: "p-2",
                                    li { a { { "Меню 1" } } }
                                    li { a { { "Меню 2" } } }
                                }
                            }
                        }
                    }
                }
            }
/*
            div { class: "navbar-center hidden md:flex [&>*]:text-lg hoover:[&>*]:rounded-sm",
                ul { class: "menu menu-horizontal hover:[&>li>a]:text-indigo-500",
                    li { a { { "Головна" } } }
                    li {
                        details {
                            summary { { "Інструктору" } }
                            ul { class: "p-2 rounded-sm",
                                li { a { { "Меню 1" } } }
                                li { a { { "Меню 2" } } }
                            }
                        }
                    }
                }
            }

 */
            div { class: "navbar-end",
                div { class: "join",
                    LanguageSwitcherComponent {}
                    ThemeSwitcherComponent {}
                    AccountControllerComponent {}
                }
            }
        }
    }
}