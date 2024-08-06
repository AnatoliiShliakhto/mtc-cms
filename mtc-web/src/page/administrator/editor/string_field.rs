use dioxus::prelude::*;

use crate::page::administrator::editor::FieldProps;

#[component]
pub fn StringField(props: FieldProps) -> Element {
    rsx! {
        label { class: "w-full form-control",
            div { class: "label",
                span { class: "label-text text-primary", { props.title } }
            }
            input { r#type: "text",
                name: props.slug,
                initial_value: props.value.as_str(),
                class: "input input-bordered",
            }
        }
    }
}
