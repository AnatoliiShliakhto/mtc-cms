use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div { class: "min-w-60 p-2 self-center",
            "DASHBOARD In-Dev"
        }
    }
}