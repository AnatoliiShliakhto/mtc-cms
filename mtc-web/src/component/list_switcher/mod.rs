use std::collections::BTreeSet;

use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct ListSwitcherComponentProps {
    pub title: String,
    pub items: Signal<BTreeSet<String>>,
    pub all: Signal<BTreeSet<String>>,
}

impl Clone for ListSwitcherComponentProps {
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            items: self.items,
            all: self.all,
        }
    }
}

#[component]
pub fn ListSwitcherComponent(props: ListSwitcherComponentProps) -> Element {
    let mut items = props.items;
    let mut all = props.all;

    let mut item_add = move |item: &String| {
        all.try_write().unwrap().remove(item);
        items.try_write().unwrap().insert(item.clone());
    };
    let mut item_remove = move |item: &String| {
        items.try_write().unwrap().remove(item);
        all.try_write().unwrap().insert(item.clone());
    };

    rsx! {
        div { class: "mt-4 rounded border p-3 collapse bg-base-100 input-bordered",
            tabindex: 0,
            div { class: "p-0 collapse-title",
                label { class: "w-full lowercase label-text text-primary", "⌘ " { props.title.clone() } }
                div { class: "flex flex-wrap content-start gap-2 pt-3",
                    for item in items() {
                        div { class: "badge badge-outline text-success hover:cursor-pointer hover:text-error",
                            onclick: move |_| item_remove(&item),
                            { item.clone() }
                        }
                    }
                }
            }
            div { class: "p-0 collapse-content",
                div{ class: "divider"}
                div { class: "flex flex-wrap content-start gap-2",
                    for item in all() {
                        div { class: "badge badge-outline hover:cursor-pointer hover:text-success",
                            onclick: move |_| item_add(&item),
                            { item.clone() }
                        }
                    }
                }
            }
        }
    }
}
