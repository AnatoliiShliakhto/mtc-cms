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