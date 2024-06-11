use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
            div { class: "min-w-60 p-2",
                "Second menu"
            }
            div { class: "w-full p-2",
                "Admin panel"
            }
    }
}