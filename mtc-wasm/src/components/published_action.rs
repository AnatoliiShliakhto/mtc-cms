use super::*;

#[component]
pub fn PublishedAction(
    #[props]
    checked: bool,
) -> Element {

    rsx! {
        div {
            class: "fixed top-80 right-0 opacity-50 sm:opacity-100 hover:opacity-100",
            label {
                class: "swap h-12 w-16 bg-base-200 hover:bg-neutral rounded-l-lg shadow-md",
                input {
                    r#type: "checkbox",
                    name: "published",
                    form: "content-edit-form",
                    initial_checked: checked,
                }
                div {
                    class: "swap-off",
                    Icon {
                        icon: Icons::EyeSlash,
                        class: "size-6 text-warning"
                    }
                }
                div {
                    class: "swap-on",
                    Icon {
                        icon: Icons::Eye,
                        class: "size-6 text-success"
                    }
                }
            }
        }
    }
}