use dioxus::dioxus_core::Element;
use dioxus::hooks::Resource;
use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::component::message_box::{MessageBoxComponent, MessageBoxComponentKind, MessageBoxComponentProps};

#[derive(Props)]
pub struct ReloadingBoxComponentProps<T: 'static>
{
    pub message: String,
    future: Resource<T>,
}

impl<T: 'static> PartialEq for ReloadingBoxComponentProps<T> {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
    }
}

impl<T> Clone for ReloadingBoxComponentProps<T> {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            future: self.future,
        }
    }
}

pub fn ReloadingBoxComponent<T: 'static>(
    mut props: ReloadingBoxComponentProps<T>,
) -> Element {
    let i18 = use_i18();

    rsx! {
        div { class: "flex h-full w-full items-center justify-center",
            div { class: "flex flex-col gap-3 w-fit",
                div { role: "alert", class: "flex flex-row p-4 gap-2 rounded border border-error text-error",
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
                    span { { props.message } }
                }
                button { class: "link link-hover self-center",
                        onclick: move |_| props.future.restart(), { translate!(i18, "messages.reload") }
                }
            }
        }
    }
}
