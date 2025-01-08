use super::*;

/// A toggle button for blocking a user.
#[component]
pub fn UserBlockAction(
    #[props]
    checked: bool,
) -> Element {

    rsx! {
        div {
            class: "fixed top-80 right-0 opacity-50 xl:opacity-100 hover:opacity-100",
            label {
                class: "swap h-12 w-16 bg-base-200 hover:bg-neutral rounded-l-lg shadow-md",
                input {
                    r#type: "checkbox",
                    name: "blocked",
                    form: "user-edit-form",
                    initial_checked: checked,
                }
                div {
                    class: "swap-off",
                    Icon {
                        icon: Icons::UserCheck,
                        class: "size-6 text-success"
                    }
                }
                div {
                    class: "swap-on",
                    Icon {
                        icon: Icons::Ban,
                        class: "size-6 text-error"
                    }
                }
            }
        }
    }
}