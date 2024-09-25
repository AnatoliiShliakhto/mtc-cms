use super::*;

#[component]
pub fn FormNumField(
    #[props(into)]
    name: String,
    #[props(into)]
    title: String,
    #[props]
    required: Option<bool>,
    #[props]
    initial_value: Option<String>,
    #[props]
    min: Option<String>,
    #[props]
    max: Option<String>,
    #[props]
    step: Option<String>,
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
                r#type: "number",
                name,
                class: "input-field",
                initial_value,
                required,
                min,
                max,
                step,
            }
        }
    }
}