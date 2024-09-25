use super::*;

pub fn PlusIcon(class: &'static str) -> Element {
    rsx! {
        svg {
            class,
            "viewBox": "0 0 24 24",
            "stroke-linecap": "butt",
            "stroke-linejoin": "miter",
            "stroke-width": "0",
            "fill": "currentColor",
            "stroke": "none",
            "xmlns": "http://www.w3.org/2000/svg",
            path { 
                "d": "M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" 
            }
        }
    }
} 