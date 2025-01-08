use super::*;

/// A component to render an HTML field.
#[component]
pub fn ViewHtmlField(
    #[props(into)]
    value: Option<String>,
) -> Element {
    let Some(inner) = value else { return rsx!{} };
    let is_web = state!(platform).eq("web");

    let inner = inner
        .as_str()
        .replace(r#"href=""#, r#"onclick="linkOpen(this); return false;" href=""#);

    rsx! {
        article {
            dangerous_inner_html: inner,
        }
    }
}