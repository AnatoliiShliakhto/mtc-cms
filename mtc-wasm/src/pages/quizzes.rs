use super::*;

/// Quiz list page provides brief quiz information and allows:
/// - create quiz
/// - assign quiz to user groups
/// - select quiz
#[component]
pub fn Quizzes() -> Element {
    breadcrumbs!("menu-quizzes");
    check_permission!(PERMISSION_QUIZZES_READ);

    let writer = state!(auth).has_permission(PERMISSION_QUIZZES_WRITE);

    let future = value_future!(url!(API_QUIZZES));
    let response = future.suspend()?;
    check_response!(response, future);

    rsx! {
        section { class: "w-full grow xl:pr-16",
            table { class: "entry-table",
                thead {
                    tr {
                        th { class: "w-12" }
                        th { class: "w-3/12", {t!("field-slug")} }
                        th { class: "w-full", {t!("field-title")} }
                    }
                }
                tbody {
                    for quiz in response().self_obj::<Vec<Entry>>().unwrap_or_default().iter() {
                        {
                            let quiz_edit_id = quiz.id.clone();
                            let quiz_assign_id = quiz.id.clone();
                            rsx! {
                                tr {
                                    onclick: move |event| {
                                        event.prevent_default();
                                        event.stop_propagation();
                                        navigator().push(route!(API_ADMINISTRATOR, API_QUIZZES, quiz_edit_id));
                                    },
                                    td {
                                        button {
                                            onclick: move |event| {
                                                event.prevent_default();
                                                event.stop_propagation();
                                                navigator()
                                                    .push(
                                                        route!(API_ADMINISTRATOR, API_QUIZZES, quiz_assign_id, API_ASSIGNMENTS),
                                                    );
                                            },
                                            name: "assign-quiz",
                                            class: "btn btn-xs btn-ghost",
                                            r#type: "button",
                                            disabled: !writer,
                                            Icon { icon: Icons::People, class: "size-4" }
                                        }
                                    }
                                    td { class: "text-neutral", {quiz.slug.as_ref()} }
                                    td { class: "text-neutral", {quiz.title.as_ref()} }
                                }
                            }
                        }
                    }
                }
                EntriesActions {
                    future,
                    route: route!(API_ADMINISTRATOR, API_QUIZZES, ID_CREATE),
                    permission: PERMISSION_QUIZZES_WRITE,
                }
            }
        }
    }
}
