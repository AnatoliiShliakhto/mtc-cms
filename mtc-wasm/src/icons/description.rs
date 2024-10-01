use super::*;

pub fn DescriptionIcon(class: &'static str) -> Element {
    rsx! {
        svg {
            class,
            "xmlns": "http://www.w3.org/2000/svg",
            "fill": "currentColor",
            "viewBox": "0 -960 960 960",
            path {
                "d": "M320-240h320v-80H320v80Zm0-160h320v-80H320v80ZM240-80q-33 \
                0-56.5-23.5T160-160v-640q0-33 23.5-56.5T240-880h320l240 240v480q0 \
                33-23.5 56.5T720-80H240Zm280-520v-200H240v640h480v-440H520ZM240-800v200-200 \
                640-640Z"
            }
        }
    }
}