use super::*;

#[component]
pub fn EditorActions(
    #[props(into)]
    form: String,
    #[props]
    delete_handler: Option<EventHandler<MouseEvent>>,
    #[props]
    permission: Option<String>,
) -> Element {
    let auth_state = use_auth_state();

    rsx! {
        div {
            class: "action-right-panel group join join-vertical top-40 \
            opacity-50 xl:opacity-100 hover:opacity-100",
            onclick: move |event| event.stop_propagation(),

            button {
                class: "hover:btn-neutral join-item",
                onclick: move |_| navigator().go_back(),
                Icon { icon: Icons::Back, class: "size-6" }
                span {
                    class: "opacity-0 group-hover:opacity-100",
                    { t!("action-back") }
                }
            }

            button {
                form,
                class: if permission.is_none()
                || auth_state().has_permission(&permission.unwrap_or_default()) {
                    "hover:btn-success join-item"
                } else {
                    "btn-disabled join-item"
                },
                onclick: move |event| event.stop_propagation(),
                Icon { icon: Icons::Floppy, class: "size-6" }
                span {
                    class: "opacity-0 group-hover:opacity-100",
                    { t!("action-save") }
                }
            }

            button {
                class: if delete_handler.is_some() {
                    "hover:btn-error join-item"
                } else {
                    "btn-disabled join-item"
                },
                onclick: move |event| {
                    if let Some(handler) = delete_handler {
                        alert_dialog!("message-confirm-deletion", handler)
                    }
                },
                Icon { icon: Icons::Trash, class: "size-6" }
                span {
                    class: "opacity-0 group-hover:opacity-100",
                    { t!("action-delete") }
                }
            }
        }
    }
}