use super::*;

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
            select { class: "select select-bordered input-bordered focus:input-primary",
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
