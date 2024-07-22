use dioxus::prelude::*;

use crate::element::editor::FieldProps;

#[component]
pub fn StringField(props: FieldProps) -> Element {
    rsx! {
        label { class: "w-full form-control",
            div { class: "label",
                span { class: "label-text text-primary", { props.title } }
            }
            input { r#type: "text",
                name: props.slug,
                value: props.value,
                class: "input input-bordered",
            }
        }
    }
}
