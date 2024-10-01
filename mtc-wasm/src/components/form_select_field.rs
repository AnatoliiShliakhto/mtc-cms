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
        label {
            class: "w-full form-control",
            div {
                class: "label",
                span {
                    class: "label-text text-neutral",
                    "⌘ " { t!(title.as_str()) }
                }
            }
            select { class: "select select-bordered input-bordered focus:input-primary",
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