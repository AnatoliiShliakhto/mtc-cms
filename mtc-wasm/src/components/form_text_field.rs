use super::*;

#[component]
pub fn FormTextField(
    #[props(into)]
    name: String,
    #[props(into)]
    title: String,
    #[props(into)]
    required: Option<bool>,
    #[props]
    disabled: Option<bool>,
    #[props]
    initial_value: Option<String>,
    #[props]
    pattern: Option<String>,
    #[props]
    r#type: Option<String>,
) -> Element {

    rsx! {
        label {
            class: "w-full floating-label mt-4",
            span {
                "⌘ " { t!(title.as_str()) }
            }
            input {
                r#type,
                name,
                class: "w-full input validator",
                initial_value,
                pattern,
                required,
                disabled,
            }
        }
    }
}