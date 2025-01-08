use super::*;

/// A form component that displays a CKEditor text area.
///
/// The component is displayed as a label with a title, and a text area
/// with a CKEditor instance. The `name` parameter is the name of the
/// text area, and the `title` parameter is the title displayed above
/// the text area. The `initial_value` parameter is the initial value
/// of the text area.
///
/// The component uses the `CkEditorCreate` function to create the
/// CKEditor instance. The instance is created when the component is
/// mounted, and the `eval` function is used to execute the
/// `CkEditorCreate` function.
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