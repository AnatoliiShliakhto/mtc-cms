use super::*;

/// Component for editing or creating a course entry.
///
/// # Props
/// - `slug`: The unique identifier for the course content.
/// - `id`: The ID of the course entry being edited or created.
/// - `is_new`: A boolean indicating if the course entry is new.
///
/// # Description
/// This component provides a form interface for editing or creating a course entry,
/// allowing the user to modify the title, description, and links associated with
/// the course. It manages the course data state and handles form submission and
/// deletion operations. The component also checks permissions and displays
/// additional actions such as publishing and storage management.
#[component]
pub fn CourseEdit(
    #[props(into)]
    slug: String,
    #[props(into)]
    id: usize,
    #[props]
    is_new: bool,
) -> Element {
    let slug = use_memo(use_reactive!(|slug| slug));
    let id = use_memo(use_reactive!(|id| id));
    let is_new = use_memo(use_reactive!(|is_new| is_new));
    let mut current_item = use_signal(CourseEntry::default);

    breadcrumbs!("menu-course-edit");
    check_permission!(PERMISSION_COURSE_WRITE);

    let future = value_future!(url!(API_CONTENT, API_COURSE, &slug()));
    let response = future.suspend()?;
    check_response!(response, future);

    let content_id = response().key_string("id").unwrap_or_default();
    let content = response().key_obj::<Value>("data").unwrap_or_default();
    let fields = response().key_obj::<Vec<Field>>("fields").unwrap_or_default();

    let course_entries = content.key_obj::<Vec<CourseEntry>>(API_COURSE)
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
        let links_arr = links_obj.self_obj::<Vec<LinkEntry>>().unwrap_or_default();
        for link in links_arr {
            links
                .push(format!("{}; {}", link.title.trim(), link.url.trim()));
        }
        links
    } else {
        vec![]
    };

    let content_id = response().key_string("id").unwrap_or_default();
    eval(&format!("window.contentId = '{content_id}'"));

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
        data.insert_value(API_COURSE, json!(course));

        let payload = json!({
            "id": event.get_str("id"),
            "published": event.get_bool("published"),
            "data": data
        });

        spawn(async move {
            if post_request!(url!(API_CONTENT, API_COURSE, &slug()), payload) {
                navigator().go_back();
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
        data.insert_value(API_COURSE, json!(course));

        let payload = json!({
            "id": content_id,
            "data": data
        });

        spawn(async move {
            if post_request!(url!(API_CONTENT, API_COURSE, &slug()), payload) {
                navigator().go_back();
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
                    initial_value: response().key_string("id")
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
            created_by: response().key_string("created_by"),
            created_at: response().key_datetime("created_at"),
            updated_by: response().key_string("updated_by"),
            updated_at: response().key_datetime("updated_at"),
        }
        if is_new() {
            EditorActions {
                form: "content-edit-form",
                permission: PERMISSION_COURSE_WRITE,
            }
        } else {
            EditorActions {
                form: "content-edit-form",
                delete_handler: delete,
                permission: PERMISSION_COURSE_WRITE,
            }
        }
        PublishedAction {
            checked: response().key_bool("published").unwrap_or_default()
        }
        StorageActions { id: response().key_string("id").unwrap_or_default() }
    }
}