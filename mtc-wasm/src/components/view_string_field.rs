use super::*;

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