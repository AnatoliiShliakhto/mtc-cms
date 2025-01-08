use super::*;

/// A component that renders a panel with actions for accessing the public
/// and private storage boxes for a given user.
#[component]
pub fn StorageActions(
    #[props(into)]
    id: String,
) -> Element {
    let auth = state!(auth);
    let mut public_box_visible = use_signal(|| false);
    let mut private_box_visible = use_signal(|| false);

    rsx! {
        div {
            class: "action-right-panel group join join-vertical top-96 \
            opacity-50 xl:opacity-100 hover:opacity-100",
            onclick: move |event| event.stop_propagation(),

            button {
                class: if auth.has_permission(PERMISSION_PUBLIC_STORAGE_READ) {
                    "hover:btn-accent join-item"
                } else {
                    "btn-disabled join-item"
                },
                onclick: move |_| public_box_visible.set(true),
                Icon { icon: Icons::Database, class: "size-6" }
                span {
                    class: "opacity-0 group-hover:opacity-100",
                    { t!("action-public-storage") }
                }
            }
            button {
                class: if auth.has_permission(PERMISSION_PRIVATE_STORAGE_READ) {
                    "hover:btn-warning join-item"
                } else {
                    "btn-disabled join-item"
                },
                onclick: move |event| private_box_visible.set(true),
                Icon { icon: Icons::DatabaseLock, class: "size-6" }
                span {
                    class: "opacity-0 group-hover:opacity-100",
                    { t!("action-private-storage") }
                }
            }
        }
        div {
            StorageBox {
                id: id.clone(),
                is_private: false,
                is_show: public_box_visible,
            }
            StorageBox {
                id: id.clone(),
                is_private: true,
                is_show: private_box_visible,
            }
        }
    }
}