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
    use_effect({
        let name = name.clone();
        move || {
            jsFfiCreateCkEditor(&name)
        }
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
            article { class: "prose-sm md:prose-base max-w-full",
                textarea {
                    id: name.clone(),
                    name: name.clone(),
                    dangerous_inner_html: initial_value,
                }
            }
        }
    }
}