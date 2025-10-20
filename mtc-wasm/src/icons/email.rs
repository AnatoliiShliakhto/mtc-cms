use super::*;

pub fn SendEmailIcon(class: &'static str) -> Element {
    rsx! {
        svg {
            class,
            "viewBox": "0 0 20 20",
            "fill": "#222222",
            "fill-opacity": "1",
            "stroke": "none",
            "stroke-width": "0px",
            "xmlns": "http://www.w3.org/2000/svg",
            path { d: "M 0 4 L 0 17 L 9 17 L 9 16 L 1.6621094 16 L 7.296875 11.753906 L 7.2988281 11.751953 L 6.46875 11.126953 L 6.4667969 11.128906 L 1 15.246094 L 1 5.9335938 L 9.0488281 12 L 10.951172 12 L 19 5.9335938 L 19 12.671875 L 20 13.671875 L 20 4 L 0 4 z M 1.6621094 5 L 18.337891 5 L 10.376953 11 L 9.6230469 11 L 1.6621094 5 z M 15 13 L 18 16 L 11 16 L 11 17 L 18 17 L 15 20 L 16.5 20 L 20 16.5 L 16.5 13 L 15 13 z " }
        }
    }
}