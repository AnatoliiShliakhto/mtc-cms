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
            class: "w-full floating-label mt-4",
            span {
                "⌘ " { t!(title.as_str()) }
            }
            textarea {
                name,
                class: "w-full h-48 textarea",
                initial_value,
                required,
            }
        }
    }
}