use super::*;

#[component]
pub fn ContentViewWithArg(
    #[props(into)]
    schema: String,
    #[props(into)]
    slug: String,
    #[props(into)]
    arg: String,
) -> Element {
    rsx! {
        ContentView {
            schema,
            slug,
            arg
        }
    }
}