use super::*;

/// A component that renders a toggle switch input field within a form.
///
/// # Props
///
/// * `name`: The name attribute of the toggle input, used for form data submission.
/// * `title`: The title displayed as a label text next to the toggle switch.
/// * `checked`: A boolean value to set the initial checked state of the toggle.
#[component]
pub fn FormToggleField(
    #[props(into)]
    name: String,
    #[props(into)]
    title: String,
    #[props]
    checked: bool,
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
            div {
                class: "label cursor-pointer justify-start gap-5",
                input {
                    r#type: "checkbox",
                    name,
                    class: "toggle toggle-primary",
                    checked,
                }
                span {
                    class: "label-text",
                    { t!("action-yes") }
                }
            }
        }
    }
}