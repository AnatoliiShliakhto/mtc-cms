use super::*;

/// A form component that displays a number input field.
///
/// The component is displayed as a label with a title, and a number input field.
/// The `name` parameter is the name of the number input field, and the `title`
/// parameter is the title displayed above the number input field. The
/// `initial_value` parameter is the initial value of the number input field.
/// The `required` parameter is an optional boolean value to set the input
/// field as required. The `min`, `max` and `step` parameters are optional
/// string values to set the minimum, maximum and step value of the number
/// input field.
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