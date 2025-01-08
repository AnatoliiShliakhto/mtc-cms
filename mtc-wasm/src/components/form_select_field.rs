use super::*;

/// A component for rendering a form select field with a label and options.
///
/// # Parameters
///
/// - `name`: The name attribute for the select element.
/// - `title`: The title to be displayed as a label for the select field.
/// - `selected`: The ID of the currently selected option.
/// - `items`: A vector of `Entry` items, each providing an ID and a title for the select options.
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