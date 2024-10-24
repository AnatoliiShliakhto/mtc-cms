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
    let ckeditor_eval = format!("CkEditorCreate('#{name}')");
    use_effect(move || {
        eval(&ckeditor_eval);
    });

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
    }
}