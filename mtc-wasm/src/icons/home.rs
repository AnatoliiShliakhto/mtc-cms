use super::*;

pub fn HomeIcon(class: &'static str) -> Element {
    rsx! {
        svg {
            class,
            "viewBox": "0 0 24 24",
            "fill": "currentColor",
            "stroke-linejoin": "miter",
            "xmlns": "http://www.w3.org/2000/svg",
            "stroke": "none",
            "stroke-width": "0",
            "stroke-linecap": "butt",
            path {
                "d": "M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z"
            }
        }
    }
} 