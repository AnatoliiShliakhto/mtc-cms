use super::*;

#[component]
pub fn ViewHtmlField(
    #[props(into)]
    value: Option<String>,
) -> Element {
    if value.is_none() {
        return rsx!{}
    }

     rsx! {
         article {
             dangerous_inner_html: value,
         }
     }
}