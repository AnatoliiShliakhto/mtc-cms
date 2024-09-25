use super::*;

pub fn ArrowUpIcon(class: &'static str) -> Element {
    rsx! {
        svg {
            class,
            "stroke-width": "0",
            "fill": "currentColor",
            "stroke-linejoin": "miter",
            "xmlns": "http://www.w3.org/2000/svg",
            "viewBox": "0 0 24 24",
            "stroke": "none",
            "stroke-linecap": "butt",
            path {
                "d": "M4 12l1.41 1.41L11 7.83V20h2V7.83l5.58 5.59L20 12l-8-8-8 8z"
            }
         }
    }
}