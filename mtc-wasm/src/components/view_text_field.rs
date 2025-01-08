use super::*;

/// A component to render a text field in a view.
#[component]
pub fn ViewTextField(
    #[props(into)]
    value: Option<String>,
) -> Element {
    if value.is_none() {
        return rsx!{}
    }

    rsx! {
        p { class: "whitespace-pre-line",
            { value }
        }
    }
}