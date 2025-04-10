use super::*;

/// A component for selecting multiple items from a list of options.
///
/// The component displays a list of badges representing the selected items,
/// and a collapsible list of all items. When an item is clicked, it is moved
/// from the collapsible list to the list of selected items.
///
/// The component takes four props:
///
/// - `name`: The name of the form field to use when submitting the selected
///   items.
/// - `title`: The label to display above the component.
/// - `items`: A vector of strings, where each string is an item to display in
///   the collapsible list.
/// - `entries`: A vector of `Entry` objects, where each object has an `id` and
///   `title` property. The `id` is used to identify the item in the
///   collapsible list, and the `title` is used to display the item in the
///   collapsible list.
///
/// The component returns an `Element` representing the component.
#[component]
pub fn FormEntriesField(
    #[props(into)]
    name: String,
    #[props(into)]
    title: String,
    #[props]
    items: Vec<Cow<'static, str>>,
    #[props]
    entries: Vec<Entry>,
) -> Element {
    let mut active_items = use_signal(|| items.iter()
        .map(|val| val.to_owned())
        .collect::<BTreeSet<Cow<'static, str>>>()
    );

    let mut all_items = use_signal(|| entries.iter()
        .filter(|val| !active_items().contains(&val.id))
        .map(|val| val.id.clone())
        .collect::<BTreeSet<Cow<'static, str>>>()
    );

    let badges = use_signal(|| entries
        .iter()
        .map(|val| (val.id.clone(), val.title.clone()))
        .collect::<BTreeMap<Cow<'static, str>, Cow<'static, str>>>()
    );

    let get_badge_title = move |id: &str| {
        match badges().get(id) {
            Some(value) => value.clone(),
            _ => id.to_string().into(),
        }
    };

    let mut badge_add = move |item: Cow<'static, str>| {
        all_items.try_write().unwrap().remove(&item);
        active_items.try_write().unwrap().insert(item);
    };
    let mut badge_remove = move |item: Cow<'static, str>| {
        active_items.try_write().unwrap().remove(&item);
        all_items.try_write().unwrap().insert(item);
    };

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
            div {
                class: "rounded border p-3 collapse bg-base-100",
                tabindex: 0,
                div {
                    class: "p-0 collapse-title",
                    div {
                        class: "flex flex-wrap content-start gap-2 pt-3",
                        for item in active_items() {{
                            let id = item.clone();
                            rsx! {
                                div {
                                    class: "badge badge-outline text-success hover:cursor-pointer hover:text-error",
                                    onclick: move |_| badge_remove(id.clone()),
                                    { get_badge_title(&item) }
                                }
                            }
                        }}
                    }
                }
                div { class: "p-0 collapse-content",
                    div{ class: "divider"}
                    div { class: "flex flex-wrap content-start gap-2",
                        for item in all_items() {{
                            let id = item.clone();
                            rsx! {
                                div { class: "badge badge-outline hover:cursor-pointer hover:text-success",
                                    onclick: move |_| badge_add(id.clone()),
                                    { get_badge_title(&item) }
                                }
                            }
                        }}
                    }
                }
            }
        }
        for item in active_items() {
            input {
                r#type: "hidden",
                name: name.clone(),
                value: item.to_string(),
            }
        }
    }
}