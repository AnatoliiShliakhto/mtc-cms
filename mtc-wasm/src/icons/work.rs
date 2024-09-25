use super::*;

pub fn WorkIcon(class: &'static str) -> Element {
    rsx! {
        svg {
            class,
            "stroke-linejoin": "miter",
            "xmlns": "http://www.w3.org/2000/svg",
            "stroke-width": "0",
            "stroke-linecap": "butt",
            "viewBox": "0 0 24 24",
            "fill": "currentColor",
            "stroke": "none",
            path {
                "fill-rule": "evenodd",
                "d": "M14 6V4h-4v2h4zM4 8v11h16V8H4zm16-2c1.11 0 2 .89 2 2v11c0 1.11-.89 \
                2-2 2H4c-1.11 0-2-.89-2-2l.01-11c0-1.11.88-2 1.99-2h4V4c0-1.11.89-2 2-2h4c1.11 \
                0 2 .89 2 2v2h4z"
            }
        }
    }
} 