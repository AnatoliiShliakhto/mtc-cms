use dioxus::prelude::*;

use crate::component::header::logo::HeaderLogo;
use crate::router::Route::HomePage;
use crate::widget::account_controller::AccountControllerWidget;
use crate::widget::theme_switcher::ThemeSwitcherWidget;

mod logo;

pub fn Header() -> Element {
    rsx! {
        div { class: "navbar bg-base-100 static shadow-lg px-2",
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
            div { class: "navbar-end",
                div { class: "join border-sm",
                    AccountControllerWidget {}
                    ThemeSwitcherWidget {}
                }
            }
        }
    }
}