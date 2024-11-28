use super::*;

pub fn TabletIcon(class: &'static str) -> Element {
    rsx! {
        svg {
            class,
            "viewBox": "0 0 16 16",
            fill: "currentColor",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M12 1a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1zM4 \
                0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2z"
            }
            path { d: "M8 14a1 1 0 1 0 0-2 1 1 0 0 0 0 2" }
        }
    }
}