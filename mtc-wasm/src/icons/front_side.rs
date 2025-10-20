use super::*;

pub fn FrontSideIcon(class: &'static str) -> Element {
    rsx! {
        svg {
            class,
            "viewBox": "0 0 17 17",
            "fill": "#000000",
            "xmlns": "http://www.w3.org/2000/svg",
            path { d: "M0 3v11h17v-11h-17zM16 13h-15v-9h15v9zM13 8h-10v-1h10v1zM8 11h-5v-2h5v2z" }
        }
    }
}