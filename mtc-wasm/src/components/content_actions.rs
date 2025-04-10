use super::*;

#[component]
pub fn ContentActions(
    #[props(into)]
    schema: String,
    #[props(into)]
    slug: String,
) -> Element {

    rsx! {
        div {
            class: "action-right-panel top-24 group opacity-50 xl:opacity-100 hover:opacity-100",
            onclick: move |event| event.stop_propagation(),
            button {
                class: "hover:btn-warning rounded-r-none",
                onclick: move |_| {
                    navigator().push(route!(API_EDITOR, &schema, &slug));
                },
                Icon { icon: Icons::Pen, class: "size-6" }
                span {
                    class: "opacity-0 group-hover:opacity-100",
                    { t!("action-edit") }
                }
            }
        }
    }
}