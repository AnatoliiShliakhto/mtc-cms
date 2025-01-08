use super::*;

/// A component that displays a bar with three buttons.
///
/// The bar is displayed on the right side of the page, and the buttons are
/// displayed vertically. The buttons are:
///
/// 1. A "back" button that navigates back to the previous page.
/// 2. A "save" button that submits the form to the server.
/// 3. A "delete" button that displays a dialog box asking the user to confirm
///    the deletion.
///
/// The buttons are only enabled if the user has the required permissions.
///
/// The `form` parameter is the name of the form to submit when the save button
/// is clicked. The `delete_handler` parameter is the handler to call when the
/// delete button is clicked. The `permission` parameter is the permission
/// required to enable the save and delete buttons.
///
/// If `permission` is `None`, the buttons are enabled if the user is an
/// administrator. Otherwise, the buttons are enabled if the user has the
/// specified permission.
#[component]
pub fn EditorActions(
    #[props(into)]
    form: String,
    #[props]
    delete_handler: Option<EventHandler<MouseEvent>>,
    #[props]
    permission: Option<String>,
) -> Element {
    let auth = state!(auth);

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
                || auth.has_permission(&permission.unwrap_or_default()) {
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