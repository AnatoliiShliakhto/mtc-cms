use super::*;

/// A component to render a form field with a text area element.
///
/// Props:
///
/// - `name`: The name of the form field.
/// - `title`: The title of the form field to display as a label.
/// - `required`: Whether the field is required or not.
/// - `initial_value`: The initial value of the form field.
#[component]
pub fn FormTextAreaField(
    #[props(into)]
    name: String,
    #[props(into)]
    title: String,
    #[props]
    required: Option<bool>,
    #[props]
    initial_value: Option<String>,
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
            textarea {
                name,
                class: "w-full h-24 rounded textarea textarea-bordered",
                initial_value,
                required,
            }
        }
    }
}