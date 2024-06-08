use dioxus::prelude::*;

pub fn MessageBoxSuccess(message: String) -> Element {
    rsx! {
        div { role: "alert", class: "flex flex-row p-4 gap-2 rounded border border-success text-success",
            svg {
                "fill": "none",
                "xmlns": "http://www.w3.org/2000/svg",
                "viewBox": "0 0 24 24",
                class: "stroke-current shrink-0 h-6 w-6",
                path {
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    "stroke-width": "2",
                    "d": "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                }
            }
            span { { message } }
        }
    }
}