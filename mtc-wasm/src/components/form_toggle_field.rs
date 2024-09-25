use super::*;

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