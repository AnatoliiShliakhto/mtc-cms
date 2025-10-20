use super::*;

pub fn ActiveBlockIcon(class: &'static str) -> Element {
    rsx! {
        svg {
                width: "40px",
                height: "40px",
                view_box: "0 0 64 64",
                xmlns: "http://www.w3.org/2000/svg",
                circle {
                    cx: "32",
                    cy: "32",
                    r: "30",
                    fill: "#db0b32",
                }
                path {
                    fill: "#ffffff",
                    d: "M9 26h46v12H9z",
                }
            }
    }
}

pub fn ExpiredBlockIcon(class: &'static str) -> Element {
    rsx! {
        svg {
                width: "40px",
                height: "40px",
                view_box: "0 0 64 64",
                xmlns: "http://www.w3.org/2000/svg",
                circle {
                    cx: "32",
                    cy: "32",
                    r: "30",
                    fill: "#1a1a1a",
                }
                path {
                    fill: "#ffffff",
                    d: "M9 26h46v12H9z",
                }
            }
    }
}

pub fn InactiveBlockIcon(class: &'static str) -> Element {
    rsx! {
        svg {
                width: "40px",
                height: "40px",
                view_box: "0 0 64 64",
                xmlns: "http://www.w3.org/2000/svg",
                circle {
                    cx: "32",
                    cy: "32",
                    r: "30",
                    fill: "#e6e1e2",
                }
                path {
                    fill: "#ffffff",
                    d: "M9 26h46v12H9z",
                }
            }
    }
}
