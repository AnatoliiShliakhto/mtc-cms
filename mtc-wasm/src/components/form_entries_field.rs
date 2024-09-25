use super::*;

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
                class: "rounded border p-3 collapse bg-base-100 input-bordered",
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