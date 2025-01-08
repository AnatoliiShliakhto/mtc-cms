use super::*;

/// A component that renders a checkbox input field within a form.
///
/// # Props
///
/// * `name`: The name attribute of the checkbox input, used for form data submission.
/// * `title`: The title displayed as a label text next to the checkbox.
/// * `initial_checked`: An optional boolean value to set the initial checked state of the checkbox.
///
/// # Returns
///
/// An `Element` representing the checkbox input field wrapped within a styled label and div, including a title and a span displaying an affirmative action text.
#[component]
pub fn FormCheckBoxField(
    #[props(into)]
    name: String,
    #[props(into)]
    title: String,
    #[props]
    initial_checked: Option<bool>,
) -> Element {

    rsx! {
        div {
            class: "flex w-full flex-col",
        div {
            class: "label",
            span {
                class: "label-text text-neutral",
                "⌘ " { t!(title.as_str()) }
            }
        }
        label {
            class: "form-control w-full cursor-pointer label justify-start gap-3",
            class: "flex flex-row flex-nowrap",
            input {
                name,
                class: "checkbox",
                r#type: "checkbox",
                initial_checked,
            }
            span {
                class: "w-full",
                { t!("action-yes") }
            }
        }
        }
    }
}