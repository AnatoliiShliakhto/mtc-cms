use super::*;

#[component]
pub fn FormFieldsField(
    #[props]
    items: Vec<Field>,
) -> Element {
    let mut fields= use_signal(|| items.clone());

    let submit = move |event: Event<FormData>| {
        fields.write().push(Field{
            kind: FieldKind::from_str(&event.get_str("kind").unwrap_or_default())
                .unwrap_or_default(),
            slug: event.get_str("slug").unwrap_or_default(),
            title: event.get_str("title").unwrap_or_default(),
        })
    };

    rsx! {
        div {
            class: "mt-1 w-full",
            div {
                class: "label",
                span {
                    class: "label-text text-neutral",
                    "⌘ " { t!("field-fields") }
                }
            }
            table {
                class: "table table-fixed w-full",
                thead {
                    tr {
                        th { class: "w-12" }
                        th {
                            class: "w-36",
                            { t!("field-kind") }
                        }
                        th { { t!("field-slug") } }
                        th { { t!("field-title") } }
                    }
                }
                tbody {
                    for (idx, field) in fields().iter().enumerate() {
                        tr {
                            class: "hover:bg-base-200 hover:shadow-md",
                            td {
                                button {
                                    class: "btn btn-xs btn-ghost text-error",
                                    onclick: move |_| {
                                        fields.write().remove(idx.clone());
                                    },
                                    Icon { icon: Icons::Close, class: "size-4 text-error" }
                                }
                            }
                            td {
                                {
                                    t!(["field-", field.kind.to_string().as_str()]
                                    .concat().as_str())
                                }
                            }
                            td { { field.slug.clone() } }
                            td { { field.title.clone() } }
                        }
                    }
                }
            }
            for field in fields() {
                input {
                    r#type: "hidden",
                    form: "schema-edit-form",
                    name: "fields-kind",
                    value : field.kind.to_string()
                }
                input {
                    r#type: "hidden",
                    form: "schema-edit-form",
                    name: "fields-slug",
                    value : field.slug.to_string()
                }
                input {
                    r#type: "hidden",
                    form: "schema-edit-form",
                    name: "fields-title",
                    value : field.title.to_string()
                }
            }
        }
        form {
            class: "w-full grid grid-cols-2 md:grid-cols-6 gap-5 mt-2",
            id: "add-field-form",
            autocomplete: "off",
            onsubmit: submit,

            label {
                class: "form-control",
                div {
                    class: "label",
                    span {
                        class: "label-text text-neutral",
                        "⌘ " { t!("field-kind") }
                    }
                }
                select {
                    class: "select select-bordered input-bordered focus:input-primary",
                    name: "kind",
                    option {
                        initial_selected: true,
                        value: "str",
                        { t!("field-str") }
                    }
                    option {
                        value: "text",
                        { t!("field-text") }
                    }
                    option {
                        value: "html",
                        { t!("field-html") }
                    }
                    option {
                        value: "links",
                        { t!("field-links") }
                    }
                    option {
                        value: "course",
                        { t!("field-course") }
                    }
                }
            }
            div {
                class: "col-span-2",
                FormTextField {
                    name: "slug",
                    title: "field-slug",
                    pattern: SLUG_PATTERN,
                    required: true
                }
            }
            div {
                class: "col-span-2",
                FormTextField {
                    name: "title",
                    title: "field-title",
                    pattern: TITLE_PATTERN,
                    required: true
                }
            }
            div {
                class: "flex justify-start items-end",
                button {
                    class: "btn btn-primary",
                    { t!("action-add") }
                }
            }
        }
    }
}