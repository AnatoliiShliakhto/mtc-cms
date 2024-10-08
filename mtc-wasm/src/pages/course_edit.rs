use super::*;

#[component]
pub fn CourseEdit(
    #[props(into)]
    slug: String,
    #[props(into)]
    id: usize,
    #[props]
    is_new: bool,
) -> Element {
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();
    let api_task = use_coroutine_handle::<ApiRequestAction>();
    let auth_state = use_auth_state();
    let api_client = use_api_client();

    page_init!("menu-course-edit", PERMISSION_COURSE_WRITE, auth_state);

    let slug = use_memo(use_reactive!(|slug| slug));
    let id = use_memo(use_reactive!(|id| id));
    let is_new = use_memo(use_reactive!(|is_new| is_new));
    let mut current_item = use_signal(CourseEntry::default);

    let future =
        use_resource(move || async move {
            request_fetch_task(url!(API_CONTENT, "course", &slug())).await
        });

    let response = future.suspend()?;
    if response().is_null() { fail!(future) }

    let content_id = response().get_string("id").unwrap_or_default();
    let content = response().get_object::<Value>("data").unwrap_or_default();
    let fields = response().get_schema_fields().unwrap_or_default();

    let course_entries =
        content.get_object::<Vec<CourseEntry>>("course")
            .unwrap_or(vec![CourseEntry::default()]);
    let course = std::sync::Arc::new(
        course_entries
            .iter()
            .map(|entry| (entry.id, entry.clone()))
            .collect::<BTreeMap<usize, CourseEntry>>()
    );

    if is_new() {
        let next_id = course.iter().map(|(id, _)| id).max().unwrap_or(&0) + 1;
        current_item.set(CourseEntry { id: next_id, parent: id(), ..Default::default() })
    } else {
        current_item.set( if let Some(item) = course.get(&id()) {
            item.clone()
        } else { CourseEntry::default() })
    }

    let links = if let Some(links_obj) = current_item().links {
        let mut links: Vec<String> = vec![];
        let links_arr =
            serde_json::from_value::<Vec<LinkEntry>>(links_obj).unwrap_or(vec![]);
        for link in links_arr {
            links
                .push(format!("{}; {}", link.title.trim(), link.url.trim()));
        }
        links
    } else {
        vec![]
    };

    SessionStorage::set("contentId", &response().get_string("id").unwrap_or_default())
        .map_err(|e| error!("{e:#?}"))
        .ok();

    let course_submit = course.clone();
    let submit = move |event: Event<FormData>| {
        let mut course = course_submit.clone().iter()
            .map(|(id, item)| (id.clone(), item.clone()))
            .collect::<BTreeMap<usize, CourseEntry>>();
        let mut data = json!({});
        let course_id = event.get_usize("course-id").unwrap_or_default();
        let links = event.get_links_array("course-links");

        let course_entry = CourseEntry {
            id: course_id.clone(),
            parent: event.get_usize("course-parent").unwrap_or_default(),
            title: event.get_str("course-title").unwrap_or_default(),
            description: event.get_str("course-description").unwrap_or_default(),
            links: if links.is_empty() { None } else {
                Some(json!(links))
            },
        };

        course.insert(course_id, course_entry);
        let course = course.values().cloned().collect::<Vec<CourseEntry>>();
        data.insert_value("course", json!(course));

        let url: String = url!(API_CONTENT, "course", &slug());
        let json_obj = json!({
            "id": event.get_str("id"),
            "published": event.get_bool("published"),
            "data": data
        });

        spawn(async move {
            match api_client()
                .post(&*url)
                .json(&json_obj)
                .send()
                .await
                .consume()
                .await {
                Ok(_) =>
                    if navigator().can_go_back() {
                        message_box_task.send(MessageBoxAction::Clear);
                        navigator().go_back()
                    } else {
                        message_box_task
                            .send(MessageBoxAction::Success(t!("message-success-post")))
                    }
                Err(e) =>
                    message_box_task.send(MessageBoxAction::Error(e.message())),
            }
        });
    };

    let course_delete_cloned = course.clone();
    let delete = move |event: MouseEvent| {
        let mut delete_ids = vec![id()];
        fn delete_item(id: usize, course: &BTreeMap<usize, CourseEntry>, ids: &mut Vec<usize>) {
            for (id, item) in course.iter()
                .filter(|(_, entry)| entry.parent.eq(&id)) {
                delete_item(id.clone(), course, ids)
            }
            ids.push(id);
        }

        let mut course = course_delete_cloned.clone().iter()
            .map(|(id, item)| (id.clone(), item.clone()))
            .collect::<BTreeMap<usize, CourseEntry>>();

        delete_item(id(), &course, &mut delete_ids);
        delete_ids.iter().for_each(|id| { course.remove(id); });

        let mut data = json!({});

        let course = course.values().cloned().collect::<Vec<CourseEntry>>();
        data.insert_value("course", json!(course));

        let url: String = url!(API_CONTENT, "course", &slug());
        let json_obj = json!({
            "id": content_id,
            "data": data
        });

        spawn(async move {
            match api_client()
                .post(&*url)
                .json(&json_obj)
                .send()
                .await
                .consume()
                .await {
                Ok(_) =>
                    if navigator().can_go_back() {
                        message_box_task.send(MessageBoxAction::Clear);
                        navigator().go_back()
                    } else {
                        message_box_task
                            .send(MessageBoxAction::Success(t!("message-success-post")))
                    }
                Err(e) =>
                    message_box_task.send(MessageBoxAction::Error(e.message())),
            }
        });
    };

    rsx! {
        section {
            class: "flex grow select-none flex-row gap-6 px-3 pr-20 sm:pr-16",
            form {
                class: "flex grow flex-col items-center gap-3",
                id: "content-edit-form",
                autocomplete: "off",
                onsubmit: submit,

                input {
                    r#type: "hidden",
                    name: "id",
                    initial_value: response().get_string("id")
                }
                input {
                    r#type: "hidden",
                    name: "course-id",
                    initial_value: current_item().id.to_string()
                }
                input {
                    r#type: "hidden",
                    name: "course-parent",
                    initial_value: current_item().parent.to_string()
                }
                FormTextField {
                    name: "course-title",
                    title: "field-title",
                    required: true,
                    initial_value: current_item().title.to_string()
                }
                FormTextAreaField {
                    name: "course-description",
                    title: "field-description",
                    initial_value: current_item().description.to_string()
                }
                FormTextAreaField {
                    name: "course-links",
                    title: "field-links",
                    initial_value: links.join("\r\n")
                }
            }
        }
        EntryInfoBox {
            created_by: response().get_string("created_by"),
            created_at: response().get_datetime("created_at"),
            updated_by: response().get_string("updated_by"),
            updated_at: response().get_datetime("updated_at"),
        }
        if is_new() {
            EditorActions {
                form: "content-edit-form",
                permission: PERMISSION_COURSE_WRITE,
            }
        } else {
            EditorActions {
                form: "content-edit-form",
                delete_event: delete,
                permission: PERMISSION_COURSE_WRITE,
            }
        }
        PublishedAction {
            checked: response().get_bool("published").unwrap_or_default()
        }
        StorageActions { id: response().get_string("id").unwrap_or_default() }
    }
}