use super::*;

#[component]
pub fn EntryInfoBox(
    #[props]
    created_by: Option<String>,
    #[props]
    created_at: Option<DateTime<Local>>,
    #[props]
    updated_by: Option<String>,
    #[props]
    updated_at: Option<DateTime<Local>>,
) -> Element {

    rsx! {
        div {
            class: "fixed top-24 right-0 dropdown dropdown-hover dropdown-left \
            opacity-50 xl:opacity-100 hover:opacity-100",
            div {
                tabindex: "0",
                role: "button",
                class: "btn text-accent rounded-l-lg rounded-r-none shadow-md hover:btn-accent",
                Icon { icon: Icons::Info2, class: "size-8" }
            }
            div {
                tabindex: "0",
                class: "dropdown-content bg-base-100 rounded border input-bordered shadow-md \
                flex flex-col min-w-44 mr-2 gap-1 label-text p-5 z-[1]",
                span {
                    class: "italic label-text text-primary",
                    { t!("field-created-at") } ":"
                }
                span { { created_by } }
                span {
                    class: "label-text-alt",
                    { created_at.unwrap_or_default().format("%H:%M %d/%m/%Y").to_string() }
                }
                span {
                    class: "mt-1 italic label-text text-primary",
                    { t!("field-updated-at") } ":"
                }
                span { { updated_by } }
                span {
                    class: "label-text-alt",
                    { updated_at.unwrap_or_default().format("%H:%M %d/%m/%Y").to_string() }
                }
            }
        }
    }
}