use super::*;

/// A component that renders a select field for selecting a permission from a list.
///
/// # Props
///
/// * `init_permission`: The initial selected permission, as a string.
/// * `permissions`: A list of available permissions, as `Cow<str>`.
#[component]
pub fn FormPermissionsField(
    #[props(into)]
    init_permission: String,
    #[props]
    permissions: Vec<Cow<'static, str>>,
) -> Element {
    rsx! {
        label {
            class: "form-control",
            div {
                class: "label",
                span {
                    class: "label-text text-neutral",
                    "⌘ " { t!("field-permission") }
                }
            }
            select {
                class: "select select-bordered input-bordered focus:input-primary",
                name: "permission",
                for permission in permissions.iter() {
                    option {
                        initial_selected:
                        permission.eq(&init_permission),
                        { permission.to_string() }
                    }
                }
            }
        }
    }
}
