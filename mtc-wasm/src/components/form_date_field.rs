use super::*;

/// A form component that displays a date input field.
///
/// The component is displayed as a label with a title, and a date input field.
/// The `name` parameter is the name of the date input field, and the `title`
/// parameter is the title displayed above the date input field. The
/// `initial_value` parameter is the initial value of the date input field.
/// The `required` parameter is an optional boolean value to set the input
/// field as required.
#[component]
pub fn FormDateField(
    #[props(into)] name: String,
    #[props(into)] title: String,
    #[props] required: Option<bool>,
    #[props] disabled: Option<bool>,
    #[props] initial_value: Option<String>,
) -> Element {
    rsx! {
        label {
            class: "w-full floating-label mt-4",
            span {
                "⌘ " { t!(title.as_str()).to_string() }
            }
            input {
                r#type: "date",
                name,
                class: "input-field",
                initial_value,
                required,
                disabled,
            }
        }
    }
}