use super::*;

/// A component to display information about a model entry (author and timestamps).
///
/// It is a dropdown box located in the top right corner of the screen.
/// The box is invisible on small screens and appears when the user hovers
/// over the area. The box contains the author and timestamps of creation and
/// update of the model entry.
///
/// The component is used in the `Entry` component and is passed the following
/// properties:
///
/// * `created_by`: The author of the model entry.
/// * `created_at`: The timestamp of creation of the model entry.
/// * `updated_by`: The author of the last update of the model entry.
/// * `updated_at`: The timestamp of the last update of the model entry.
///
/// The component returns an `Element` that can be used in the render method
/// of another component.
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
            class: "fixed top-24 right-0 dropdown dropdown-hover dropdown-left",
            div {
                tabindex: "0",
                role: "button",
                class: "btn text-accent rounded-l-lg rounded-r-none shadow-md hover:btn-accent",
                class: "opacity-50 xl:opacity-100 hover:opacity-100",
                Icon { icon: Icons::Info2, class: "size-8" }
            }
            div {
                tabindex: "0",
                class: "dropdown-content bg-base-100 rounded border shadow-md \
                flex flex-col min-w-44 mr-2 gap-1 label-text p-5 z-[10] opacity-100",
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