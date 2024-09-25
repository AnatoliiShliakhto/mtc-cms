use super::*;

#[component]
pub fn FormTextField(
    #[props(into)]
    name: String,
    #[props(into)]
    title: String,
    #[props]
    required: Option<bool>,
    #[props]
    initial_value: Option<String>,
    #[props]
    pattern: Option<String>,
    #[props]
    r#type: Option<String>,
) -> Element {

    rsx! {
        label {
            class: "w-full form-control",
            div {
                class: "label",
                span {
                    class: "label-text text-neutral",
                    "⌘ " { t!(title.as_str()) }
                }
            }
            input {
                r#type,
                name,
                class: "input-field",
                initial_value,
                pattern,
                required,
            }
        }
    }
}