use super::*;

/// Quiz assign page allows to assign quiz to particular groups of users.
/// # Arguments
/// * `id`: represents quiz id
#[component]
pub fn QuizAssign(#[props(into)] id: String) -> Element {
    breadcrumbs!("menu-quizzes");
    check_permission!(PERMISSION_QUIZZES_WRITE);

    let id = use_memo(use_reactive!(|id| id));
    let on_submit_quiz_assignment_form = move |event: Event<FormData>| {
        let payload = json!({
            "group_ids": event.get_str_array("group_ids"),
            "active_at": event.get_str("active_at"),
            "expired_at": event.get_str("expired_at"),
        });

        spawn(async move {
            post_request!(
                url!(API_QUIZZES, id.read().as_ref(), API_ASSIGNMENTS),
                payload
            );
        });
    };

    rsx! {
        form {
            class: "flex grow flex-col items-center gap-3",
            id: "quiz-assignment-form",
            autocomplete: "off",
            onsubmit: on_submit_quiz_assignment_form,
            FormEntriesField {
                name: "group_ids",
                title: "field-groups",
                items: vec![],
                entries: state!(groups),
            }
            FormDateField { name: "active_at", title: "field-active-at", required: true }
            FormDateField {
                name: "expired_at",
                title: "field-expired-at",
                required: true,
            }
            EditorActions {
                form: "quiz-assignment-form",
                permission: PERMISSION_QUIZZES_WRITE,
            }
        }
    }
}
