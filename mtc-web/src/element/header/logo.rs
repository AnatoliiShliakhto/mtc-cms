#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::router::Route::HomePage;

#[component]
pub fn HeaderLogo() -> Element {
    rsx! {
        div { class: "hidden md:flex",
            Link { class: "text-xl btn btn-ghost", to: HomePage {},
                "MTC-CMS" }
        }
    }
}
