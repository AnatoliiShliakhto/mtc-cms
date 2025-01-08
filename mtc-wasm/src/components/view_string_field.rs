use super::*;

/// A component for rendering a string as a span element.
#[component]
pub fn ViewStringField(
    #[props(into)]
    value: Option<String>,
) -> Element {
    if value.is_none() {
        return rsx!{}
    }

    rsx! {
         span {
             { value }
         }
     }
}