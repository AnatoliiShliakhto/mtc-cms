use super::*;

#[component]
pub fn ViewCourseField(
    schema: Memo<String>,
    value: Option<Value>,
    arg: Option<String>,
) -> Element {
    if value.is_none() {
        return rsx!{}
    }

    let value = use_memo(use_reactive!(|value| value.unwrap_or_default()));
    let mut arg = use_signal(||
        arg.unwrap_or_default().parse::<usize>().unwrap_or_default()
    );

    let auth_state = use_auth_state();
    let is_writer = auth_state().has_role(ROLE_WRITER);

    let course_entries: Vec<CourseEntry> =
        serde_json::from_value(value()).unwrap_or(vec![CourseEntry::default()]);
    let course = std::sync::Arc::new(
        course_entries
            .iter()
            .map(|entry| (entry.id, entry.clone()))
            .collect::<BTreeMap<usize, CourseEntry>>()
    );

    if !course.contains_key(&arg()) {
        return rsx!{}
    }

    let current_entry = course.get(&arg()).unwrap().clone();

    let mut course_tree: Vec<(String, usize)> = vec![];
    let mut cursor = arg();
    if arg() > 0 {
        while cursor > 0 {
            for (id, item) in course.iter() {
                if let Some(childs) = &item.childs {
                    if childs.iter().any(|value| value == &cursor) {
                        course_tree.insert(0, (item.title.to_string(), id.to_owned()));
                        cursor = id.to_owned();
                        break
                    }
                }
            }
        }
    }

    rsx! {
        if arg() > 0 {
            div {
                class: "flex w-full flex-col gap-2",
                for (count, item) in course_tree.iter().enumerate() {{
                    let count = count.to_owned();
                    let item = item.clone();
                    rsx! {
                        a { class: if count > 0 {
                            "link link-hover text-wrap"
                        } else {
                            "link link-hover text-lg font-semibold text-wrap"
                        },
                            style: format!("margin-left: {}rem;", count * 1),
                            onclick: move |_| arg.set(item.1),
                            { item.0.clone() }
                        }
                    }}
                }
            }
            div { class: "divider" }
        }

        if is_writer {
            div {
                class: "flex w-full flex-nowrap justify-end gap-2",
                button {
                    class: "btn btn-sm btn-outline btn-accent",
                    onclick: move |_| {
//                        navigator().push(Route::CourseEditorPage { schema_prop: schema_slug().clone(), course_id_prop: course_id().to_string(), is_new_prop: true });
                    },
                    Icon { icon: Icons::Plus, class: "size-4" }
                }
                button {
                    class: "btn btn-sm btn-outline btn-warning",
                    onclick: move |_| {
    //                        navigator().push(Route::CourseEditorPage { schema_prop: schema_slug().clone(), course_id_prop: course_id().to_string(), is_new_prop: false });
                    },
                    Icon { icon: Icons::Pen, class: "size-4" }
                }
            }
        }

        if !current_entry.title.is_empty() {
            div { class: "text-lg font-medium gap-2",
                { current_entry.title.clone() }
            }
        }
        if !current_entry.description.is_empty() {
            p { class: "whitespace-pre-line ml-3 text-sm",
                { current_entry.description.clone() }
            }
        }
        if current_entry.links.is_some() {
            div { class: "ml-3",
                ViewLinksField {
                    value: current_entry.links
                }
            }
        }
        if let Some(childs) = current_entry.childs {
            div {
                class: "course-childs",
                for child in childs.iter() {
                    { CourseChildView(&course, child.clone(), arg(), schema, is_writer) }
                }
            }
        }
    }
}

fn CourseChildView(
    course: &std::sync::Arc<BTreeMap<usize, CourseEntry>>,
    id: usize,
    parent_id: usize,
    schema: Memo<String>,
    is_writer: bool,
) -> Element {
    if !course.contains_key(&id) {
        return rsx!{}
    }

    let current_entry = course.get(&id).unwrap().clone();

    rsx! {
        div {
            class: "collapse collapse-arrow",
            input {
                r#type: "radio",
                name: parent_id.to_string()
            }
            div {
                class: "collapse-title font-semibold gap-2",
                { current_entry.title.as_ref() }
            }
            div { class: "collapse-content ml-3 pr-0",
                if is_writer {
                    div {
                        class: "flex w-full flex-nowrap justify-end gap-2",
                        button {
                            class: "btn btn-sm btn-outline btn-accent",
                            onclick: move |_| {
                                //                        navigator().push(Route::CourseEditorPage { schema_prop: schema_slug().clone(), course_id_prop: course_id().to_string(), is_new_prop: true });
                            },
                            Icon { icon: Icons::Plus, class: "size-4" }
                        }
                        button {
                            class: "btn btn-sm btn-outline btn-warning",
                            onclick: move |_| {
                                //                        navigator().push(Route::CourseEditorPage { schema_prop: schema_slug().clone(), course_id_prop: course_id().to_string(), is_new_prop: false });
                            },
                            Icon { icon: Icons::Pen, class: "size-4" }
                        }
                    }
                }
                if !current_entry.description.is_empty() {
                    p {
                        class: "whitespace-pre-line text-sm",
                        { current_entry.description.as_ref() }
                    }
                }
                if current_entry.links.is_some() {
                    div { class: "prose prose-base mt-4 max-w-full",
                        ViewLinksField {
                            value: current_entry.links
                        }
                    }
                }
                if let Some(childs) = current_entry.childs {
                    div { class: "program-childs",
                        for child in childs.iter() {
                            { CourseChildView(&course, *child, id, schema, is_writer) }
                        }
                    }
                }
            }
        }
    }
}