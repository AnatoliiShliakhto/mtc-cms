use super::*;

#[component]
pub fn FormHtmlField(
    #[props(into)]
    name: String,
    #[props(into)]
    title: String,
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
                    "⌘ " { title }
                }
            }
            article { class: "prose prose-sm md:prose-base max-w-full",
                textarea {
                    id: name.clone(),
                    name: name.clone(),
                    dangerous_inner_html: initial_value,
                }
            }
        }
        script {
            r#type: "module",
            { EVAL_CKEDITOR.to_string().replace("{field_name}", &name) }
        }
    }
}