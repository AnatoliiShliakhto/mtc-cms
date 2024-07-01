#![allow(dead_code)]

use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct MessageBoxComponentProps {
    pub kind: MessageBoxComponentKind,
}

#[derive(Clone, PartialEq)]
pub enum MessageBoxComponentKind {
    Alert(String),
    Info(String),
    Error(String),
    Success(String),
    Warning(String),
}

pub fn MessageBoxComponent(props: MessageBoxComponentProps) -> Element {
    match props.kind {
        MessageBoxComponentKind::Alert(message) => {
            rsx! {
                div { role: "alert", class: "flex flex-row gap-2 rounded border p-4 input-bordered",
                    svg {
                        "fill": "none",
                        "xmlns": "http://www.w3.org/2000/svg",
                        "viewBox": "0 0 24 24",
                        class: "stroke-info shrink-0 w-6 h-6",
                        path {
                            "stroke-linejoin": "round",
                            "stroke-width": "2",
                            "d": "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                            "stroke-linecap": "round"
                        }
                    }
                    span { { message } }
                }
            }
        }
        MessageBoxComponentKind::Info(message) => {
            rsx! {
                div { role: "alert", class: "flex flex-row gap-2 rounded border p-4 border-info text-info",
                    svg {
                        "fill": "none",
                        "xmlns": "http://www.w3.org/2000/svg",
                        "viewBox": "0 0 24 24",
                        class: "stroke-current shrink-0 w-6 h-6",
                        path {
                            "stroke-linejoin": "round",
                            "d": "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                            "stroke-linecap": "round",
                            "stroke-width": "2"
                        }
                    }
                    span { { message } }
                }
            }
        }
        MessageBoxComponentKind::Error(message) => {
            rsx! {
                div { role: "alert", class: "flex flex-row gap-2 rounded border p-4 border-error text-error",
                    svg {
                        "xmlns": "http://www.w3.org/2000/svg",
                        "fill": "none",
                        "viewBox": "0 0 24 24",
                        class: "stroke-current shrink-0 h-6 w-6",
                        path {
                            "stroke-linecap": "round",
                            "stroke-width": "2",
                            "d": "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
                            "stroke-linejoin": "round"
                        }
                    }
                    span { { message } }
                }
            }
        }
        MessageBoxComponentKind::Success(message) => {
            rsx! {
                div { role: "alert", class: "flex flex-row gap-2 rounded border p-4 border-success text-success",
                    svg {
                        "fill": "none",
                        "xmlns": "http://www.w3.org/2000/svg",
                        "viewBox": "0 0 24 24",
                        class: "stroke-current shrink-0 h-6 w-6",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            "stroke-width": "2",
                            "d": "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                        }
                    }
                    span { { message } }
                }
            }
        }
        MessageBoxComponentKind::Warning(message) => {
            rsx! {
                div { role: "alert", class: "flex flex-row gap-2 rounded border p-4 border-warning text-warning",
                    svg {
                        "viewBox": "0 0 24 24",
                        "fill": "none",
                        "xmlns": "http://www.w3.org/2000/svg",
                        class: "stroke-current shrink-0 h-6 w-6",
                        path {
                            "stroke-linejoin": "round",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "d": "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                        }
                    }
                    span { { message } }
                }
            }
        }
    }
}