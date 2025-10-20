use dioxus::prelude::*;

#[component]
pub fn NavigationLoop() -> Element {
    navigator().go_back();
    rsx! {}
}
