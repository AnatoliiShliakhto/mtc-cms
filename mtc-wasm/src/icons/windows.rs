use super::*;

pub fn WindowsIcon(class: &'static str) -> Element {
    rsx! {
        svg {
            class,
            xmlns: "http://www.w3.org/2000/svg",
            fill: "currentColor",
            "viewBox": "0 0 16 16",
            path {
                d: "M6.555 1.375 0 2.237v5.45h6.555zM0 13.795l6.555.933V8.313H0zm7.278-5.4.026 \
                6.378L16 16V8.395zM16 0 7.33 1.244v6.414H16z"
            }
        }
    }
}