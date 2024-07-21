use dioxus::prelude::*;

use crate::page::administrator::editor::FieldProps;

#[component]
pub fn TextField(props: FieldProps) -> Element {
    rsx! {
        label { class: "w-full form-control",
            div { class: "label",
                span { class: "label-text text-primary", { props.title } }
            }
            textarea {
                class: "w-full h-24 rounded textarea textarea-bordered",
                name: props.slug,
                value: props.value,
            }
        }
    }
}
