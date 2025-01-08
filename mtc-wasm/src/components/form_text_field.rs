use super::*;

/// A component that renders a text input field within a form.
///
/// # Props
///
/// * `name` - The name attribute for the input field, used for form submission.
/// * `title` - The label text displayed above the input field.
/// * `required` - An optional boolean indicating whether the field is required.
/// * `initial_value` - An optional string specifying the initial value of the input field.
/// * `pattern` - An optional regex pattern that the input field's value should match.
/// * `r#type` - An optional string specifying the input type (e.g., "text", "email").
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