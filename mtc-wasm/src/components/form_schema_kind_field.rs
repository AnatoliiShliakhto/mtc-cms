use super::*;

/// A form component that displays a select input field for selecting the
/// schema type.
///
/// The component is displayed as a label with a title, and a select input
/// field with options for the different schema types. The `init_kind` parameter
/// is the initial value of the select input, and the `disabled` parameter
/// determines whether the select input is disabled.
///
/// The component uses the `SchemaKind` enum to generate the options for the
/// select input.
#[component]
pub fn FormSchemaKindField(
    #[props]
    init_kind: SchemaKind,
    #[props]
    disabled: bool,
) -> Element {
    rsx! {
        label {
            class: "form-control",
            div {
                class: "label",
                span {
                    class: "label-text text-neutral",
                    "⌘ " { t!("field-schema-type") }
                }
            }
            select { class: "select focus:input-primary",
                name: "kind",
                disabled,
                option {
                    initial_selected:
                    init_kind.eq(&SchemaKind::Page),
                    value: SchemaKind::Page.to_string(),
                    { t!("field-page") }
                }
                option {
                    initial_selected:
                    init_kind.eq(&SchemaKind::Pages),
                    value: SchemaKind::Pages.to_string(),
                    { t!("field-pages") }
                }
                option {
                    initial_selected:
                    init_kind.eq(&SchemaKind::Course),
                    value: SchemaKind::Course.to_string(),
                    { t!("field-course") }
                }
            }
        }
    }
}
