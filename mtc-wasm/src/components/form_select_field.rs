use super::*;

#[component]
pub fn FormSelectField(
    #[props(into)]
    name: String,
    #[props(into)]
    title: String,
    #[props(into)]
    selected: String,
    #[props]
    items: Vec<Entry>,
) -> Element {
    rsx! {
        fieldset {
            class: "w-full fieldset mt-4",
            legend {
                class: "fieldset-legend",
                "⌘ " { t!(title.as_str()) }
            }
            select {
                class: "select",
                name,
                option {
                    initial_selected: selected.is_empty(),
                    value: "".to_string(),
                    { t!("field-selected-none") }
                }
                for item in items {
                    option {
                        initial_selected: selected.eq(&item.id),
                        value: &*item.id,
                        { &*item.title }
                    }
                }
            }
        }
    }
}

#[component]
pub fn FormSimpleSelectField(
    #[props(into)] name: String,
    #[props(into)] title: String,
    #[props(into)] selected: String,
    #[props] required: Option<bool>,
    #[props] disabled: Option<bool>,
    #[props] items: Vec<(String, String)>,
) -> Element {
    rsx! {
        label {
            class: "w-full floating-label mt-4",
            span {
                "⌘ " { t!(title.as_str()) }
            }
            select {
                class: "select",
                required,
                disabled,
                name,
                if !required.unwrap_or_default() {
                    option {
                        initial_selected: selected.is_empty(),
                        value: "".to_string(),
                        { t!("field-selected-none") }
                    }
                }
                for (item_value, item_description) in items {
                    option {
                        initial_selected: selected.eq(&item_value),
                        value: item_value.clone(),
                        { t!(item_description.as_str()) }
                    }
                }
            }
        }
    }
}