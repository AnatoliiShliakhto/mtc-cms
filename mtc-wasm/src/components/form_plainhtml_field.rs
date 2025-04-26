use super::*;

#[component]
pub fn FormPlainHtmlField(
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