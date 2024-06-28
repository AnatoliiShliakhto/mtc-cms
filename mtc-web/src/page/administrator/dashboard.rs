use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div { class: "flex flex-col p-2 self-center grow",
            "DASHBOARD In-Dev"
        }
    }
}