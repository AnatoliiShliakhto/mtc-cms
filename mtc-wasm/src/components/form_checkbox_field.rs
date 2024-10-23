use super::*;

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