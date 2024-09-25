use super::*;

pub fn MenuIcon(class: &'static str) -> Element {
    rsx! {
        svg {
            class,
            "fill": "currentColor",
            "viewBox": "0 0 24 24",
            "xmlns": "http://www.w3.org/2000/svg",
            "stroke": "currentColor",
            path {
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                "stroke-width": "2",
                "d": "M4 6h16M4 12h8m-8 6h16"
            }
        }
    }
}