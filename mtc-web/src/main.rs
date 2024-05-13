#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_regular_icons::{FaUser};
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        NavBar {}
        div { class: "flex flex-grow",
            Router::<Route> {}
        }
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Login {}
    }
}

#[component]
fn UserAccount() -> Element {
    rsx! {
        div { class: "flex-none",
            a { class: "btn btn-ghost",
                Icon {
                    width: 20,
                    height: 20,
                    fill: "currentColor",
                    icon: FaUser
                }
            }
        }
    }
}

#[component]
fn ThemeSwitcher() -> Element {
    rsx! {
        div { class: "btn btn-ghost",
            label { class: "swap swap-rotate",
                input {
                    value: "light",
                    r#type: "checkbox",
                    class: "theme-controller"
                }
                svg {
                    "xmlns": "http://www.w3.org/2000/svg",
                    "viewBox": "0 0 24 24",
                    class: "swap-off fill-current w-6 h-6",
                    path { "d": "M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" }
                }
                svg {
                    "viewBox": "0 0 24 24",
                    "xmlns": "http://www.w3.org/2000/svg",
                    class: "swap-on fill-current w-6 h-6",
                    path { "d": "M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" }
                }
            }
        }
    }
}

#[component]
fn HeaderLogo() -> Element {
    rsx! {
        div { class: "hidden md:flex",
            a { class: "btn btn-ghost text-xl", "MTC-CMS" }
        }
    }
}

#[component]
fn NavBar() -> Element {
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
                        li { a { { "Головна" } } }
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
                UserAccount {}
                ThemeSwitcher {}
            }
        }
    }
}

#[component]
fn Login() -> Element {
    rsx! {
        div { class: "flex flex-col m-auto gap-4 p-10 rounded-lg my-10 shadow-lg hover:shadow-xl",
            label { class: "input input-bordered flex items-center gap-2",
                svg {
                    "fill": "currentColor",
                    "viewBox": "0 0 16 16",
                    "xmlns": "http://www.w3.org/2000/svg",
                    class: "w-4 h-4 opacity-70",
                    path { "d": "M8 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6ZM12.735 14c.618 0 1.093-.561.872-1.139a6.002 6.002 0 0 0-11.215 0c-.22.578.254 1.139.872 1.139h9.47Z" }
                }
                input { placeholder: "логін", r#type: "text", class: "grow" }
            }
            label { class: "input input-bordered flex items-center gap-2",
                svg {
                    "fill": "currentColor",
                     "xmlns": "http://www.w3.org/2000/svg",
                    "viewBox": "0 0 16 16",
                    class: "w-4 h-4 opacity-70",
                    path {
                        "fill-rule": "evenodd",
                        "d": "M14 6a4 4 0 0 1-4.899 3.899l-1.955 1.955a.5.5 0 0 1-.353.146H5v1.5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1-.5-.5v-2.293a.5.5 0 0 1 .146-.353l3.955-3.955A4 4 0 1 1 14 6Zm-4-2a.75.75 0 0 0 0 1.5.5.5 0 0 1 .5.5.75.75 0 0 0 1.5 0 2 2 0 0 0-2-2Z",
                        "clip-rule": "evenodd"
                    }
                }
                input { r#type: "password", placeholder: "пароль", class: "grow" }
            }
        }
    }
}