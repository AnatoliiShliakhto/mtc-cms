use super::*;

/// Quiz edit page provides detailed quiz information and allows:
/// - edit quiz
/// - create category
/// - edit category
/// - delete category
/// - create question
/// - edit question
/// - delete question
/// - create answer
/// - edit answer
/// - delete answer
///
/// View consists of:
/// - quiz editor view [fn@QuizEditorView]
///     - category list view [fn@CategoryListView]
/// - category editor view [fn@CategoryEditorView]
///     - question list view [fn@QuestionListView]
///     - question editor view [fn@QuestionEditorView]
///
/// Changes become permanent only after saving the quiz, until then they only exist on the client side.
///
/// # Arguments
/// * `id`: represents quiz id
#[component]
pub fn QuizEdit(#[props(into)] id: String) -> Element {
    breadcrumbs!("menu-quizzes");
    check_permission!(PERMISSION_QUIZZES_READ);

    let id = use_memo(use_reactive!(|id| id));
    let new_quiz = use_memo(move || id().eq(ID_CREATE));
    let writer = state!(auth).has_permission(PERMISSION_QUIZZES_WRITE);

    let mut quiz_value = if new_quiz() {
        json!({})
    } else {
        let resource = value_future!(url!(API_QUIZZES, &id()));
        let existing_quiz = resource.suspend()?;
        check_response!(existing_quiz, resource);
        existing_quiz()
    };

    let mut categories = use_signal(|| categories(&mut quiz_value));

    // Category Editor props
    let mut category_editor_visibility = use_signal(|| false);
    let mut category_editor_item_index = use_signal(|| None);
    let mut category_editor_item = use_signal(|| Category::default());

    let hide_category_editor = move || {
        category_editor_item_index.set(None);
        category_editor_item.set(Category::default());
        category_editor_visibility.set(false);
    };

    let on_select_category = move |category_index: Option<usize>| {
        if let Some(index) = category_index {
            let category_clone = categories
                .read()
                .get(index)
                .map(|category_signal: &Signal<Category>| category_signal.cloned())
                .unwrap_or_default();
            category_editor_item.set(category_clone);
            category_editor_item_index.set(category_index);
        } else {
            category_editor_item.set(Category::default());
            category_editor_item_index.set(None);
        }
        category_editor_visibility.set(true);
    };

    let on_add_category = move |category: Category| {
        let category = Signal::new(category);
        if let Some(index) = category_editor_item_index() {
            categories.write()[index] = category;
        } else {
            categories.write().push(category);
        }
        hide_category_editor.clone()();
    };

    let on_cancel_category = move |()| {
        hide_category_editor.clone()();
    };

    let on_delete_category = move |category_index: usize| {
        categories.write().remove(category_index);
    };

    rsx! {
        if category_editor_visibility() {
            CategoryEditorView {
                writer,
                category_editor_item,
                category_editor_item_index,
                on_add_category,
                on_cancel_category,
            }
        }
        div { style: display_if(!category_editor_visibility()),
            QuizEditorView {
                id,
                new_quiz,
                quiz_value,
                categories,
                category_list_view: rsx! {
                    CategoryListView {
                        writer,
                        categories,
                        on_select_category,
                        on_delete_category,
                    }
                },
            }
        }
    }
}

fn categories(quiz_value: &mut Value) -> Vec<Signal<Category>> {
    let categories: Vec<Category> = quiz_value
        .as_object_mut()
        .unwrap()
        .remove("categories")
        .and_then(|categories| serde_json::from_value(categories).ok())
        .unwrap_or_default();
    categories
        .iter()
        .map(|category| Signal::new(category.clone()))
        .collect()
}

fn display_if(condition: bool) -> String {
    format!("display:{}", if condition { "block" } else { "none" })
}

#[component]
fn QuizEditorView(
    id: Memo<String>,
    new_quiz: Memo<bool>,
    quiz_value: Value,
    mut categories: Signal<Vec<Signal<Category>>>,
    category_list_view: Element,
) -> Element {
    let on_submit_quiz = move |event: Event<FormData>| {
        let payload = json!({
            "id": event.get_str("id"),
            "slug": event.get_str("slug"),
            "title": event.get_str("title"),
            "description": event.get_str("description"),
            "duration_ms": event.get_i64("duration_min").map(|value| value * (60 * 1000)),
            "scoring_system": event.get_str("scoring_system"),
            "success_score": event.get_i64("success_score"),
            "categories": categories().iter().map(|category_signal| category_signal()).collect::<Vec<Category>>(),
        });

        spawn(async move {
            let request_result = if new_quiz() {
                post_request!(url!(API_QUIZZES), payload)
            } else {
                patch_request!(url!(API_QUIZZES, &id()), payload)
            };
            if request_result {
                navigator().replace(route!(API_ADMINISTRATOR, API_QUIZZES));
            }
        });
    };

    let on_delete_quiz = move |_| {
        spawn(async move {
            if delete_request!(url!(API_QUIZZES, &id())) {
                navigator().replace(route!(API_ADMINISTRATOR, API_QUIZZES));
            }
        });
    };

    rsx! {
        form {
            class: "flex grow flex-col items-center gap-3",
            id: "quiz-edit-form",
            autocomplete: "off",
            onsubmit: on_submit_quiz,

            input {
                r#type: "hidden",
                name: "id",
                initial_value: quiz_value.key_string("id"),
            }
            FormTextField {
                name: "slug",
                title: "field-slug",
                pattern: SLUG_PATTERN,
                required: true,
                initial_value: quiz_value.key_string("slug"),
            }
            FormTextField {
                name: "title",
                title: "field-title",
                pattern: TITLE_PATTERN,
                required: true,
                initial_value: quiz_value.key_string("title"),
            }
            FormTextField {
                name: "description",
                title: "field-description",
                required: true,
                initial_value: quiz_value.key_string("description"),
            }
            FormNumField {
                name: "duration_min",
                title: "field-duration",
                min: "1",
                max: "1000000",
                step: "1",
                required: true,
                initial_value: quiz_value
                    .key_i64("duration_ms")
                    .map(|value| value / (60 * 1000))
                    .unwrap_or(60)
                    .to_string(),
            }
            FormSimpleSelectField {
                name: "scoring_system",
                title: "field-scoring-system",
                selected: quiz_value.key_string("scoring_system").unwrap_or_default(),
                items: vec![
                    (
                        ScoringSystem::TotalSuccessScore.name(),
                        "enum-total-success-score".to_string(),
                    ),
                    (
                        ScoringSystem::TotalSuccessScoreRate.name(),
                        "enum-total-success-score-rate".to_string(),
                    ),
                    (
                        ScoringSystem::CategorySuccessScore.name(),
                        "enum-category-success-score".to_string(),
                    ),
                    (
                        ScoringSystem::CategorySuccessScoreRate.name(),
                        "enum-category-success-score-rate".to_string(),
                    ),
                ],
            }
            FormNumField {
                name: "success_score",
                title: "field-success-score",
                min: "1",
                max: "1000000",
                step: "1",
                required: true,
                initial_value: quiz_value.key_i64("success_score").unwrap_or(100).to_string(),
            }
        }
        {category_list_view}
        EntryInfoBox {
            created_by: quiz_value.key_string("created_by"),
            created_at: quiz_value.key_datetime("created_at"),
            updated_by: quiz_value.key_string("updated_by"),
            updated_at: quiz_value.key_datetime("updated_at"),
        }
        if new_quiz() {
            EditorActions { form: "quiz-edit-form", permission: PERMISSION_QUIZZES_WRITE }
        } else {
            EditorActions {
                form: "quiz-edit-form",
                delete_handler: on_delete_quiz,
                permission: PERMISSION_QUIZZES_WRITE,
            }
        }
    }
}

#[component]
fn CategoryListView(
    writer: bool,
    categories: Signal<Vec<Signal<Category>>>,
    on_select_category: EventHandler<Option<usize>>,
    on_delete_category: EventHandler<usize>,
) -> Element {
    rsx! {
        div {
            div { style: "margin-top: 20px", class: "font-semibold gap-2",
                {t!("caption-categories")}
            }
            section { class: "w-full grow xl:pr-16",
                table { class: "entry-table",
                    thead {
                        tr {
                            th { class: "w-12" }
                            th { {t!("field-title")} }
                            th { {t!("field-sample-size")} }
                            th { {t!("field-success-score")} }
                            th { {t!("field-number-of-questions")} }
                        }
                    }
                    tbody {
                        for (category_index , category) in categories().iter().copied().enumerate() {
                            CategoryRowView {
                                writer,
                                category_index,
                                category,
                                on_select_category,
                                on_delete_category,
                            }
                        }
                    }
                }
                button {
                    onclick: move |event| {
                        event.prevent_default();
                        event.stop_propagation();
                        on_select_category.call(None);
                    },
                    name: "add-category",
                    class: "btn btn-xs btn-ghost",
                    r#type: "button",
                    disabled: !writer,
                    Icon { icon: Icons::Plus, class: "size-4" }
                    {t!("action-add")}
                }
            }
        }
    }
}

#[component]
fn CategoryRowView(
    writer: bool,
    category_index: usize,
    category: Signal<Category>,
    on_select_category: EventHandler<Option<usize>>,
    on_delete_category: EventHandler<usize>,
) -> Element {
    let category_reader = category.read();
    rsx! {
        tr {
            onclick: move |_| {
                on_select_category.call(Some(category_index));
            },
            td {
                button {
                    onclick: move |event| {
                        event.prevent_default();
                        event.stop_propagation();
                        on_delete_category.call(category_index);
                    },
                    name: "delete-category",
                    class: "btn btn-xs btn-ghost",
                    r#type: "button",
                    disabled: !writer,
                    Icon { icon: Icons::Close, class: "size-4 text-error" }
                }
            }
            td { {category_reader.title.to_string()} }
            td { {category_reader.sample_size.to_string()} }
            td { {category_reader.success_score.unwrap_or_default().to_string()} }
            td { {category_reader.questions.len().to_string()} }
        }
    }
}

#[component]
fn CategoryEditorView(
    writer: bool,
    category_editor_item: Signal<Category>,
    category_editor_item_index: Signal<Option<usize>>,
    on_add_category: EventHandler<Category>,
    on_cancel_category: EventHandler<()>,
) -> Element {
    let category = category_editor_item;
    let category_reader = category.read();
    let new_category = use_memo(move || category_editor_item_index().is_none());

    let questions = category
        .read()
        .questions
        .iter()
        .map(|question| Signal::new(question.clone()))
        .collect::<Vec<Signal<Question>>>();
    let mut questions = use_signal(|| questions);

    let on_submit_category_form = move |event: Event<FormData>| {
        let category = Category {
            id: None,
            title: event.get_str("title").unwrap_or_default(),
            success_score: event.get_i64("success_score").map(|value| value as u16),
            sample_size: event
                .get_i64("sample_size")
                .map(|value| value as u16)
                .unwrap_or_default(),
            questions: questions
                .read()
                .iter()
                .map(|question_signal| question_signal())
                .collect(),
        };
        on_add_category.call(category);
    };

    // Question Editor props
    let mut question_editor_visibility = use_signal(|| false);
    let mut question_editor_item = use_signal(|| Question::default());
    let mut question_editor_item_index = use_signal(|| None::<usize>);

    let hide_question_editor = move || {
        question_editor_item_index.set(None);
        question_editor_item.set(Question::default());
        question_editor_visibility.set(false);
    };

    let on_select_question = move |question_index: Option<usize>| {
        if let Some(index) = question_index {
            let question_clone = questions
                .read()
                .get(index)
                .map(|question_signal: &Signal<Question>| question_signal.cloned())
                .unwrap_or_default();
            question_editor_item.set(question_clone);
            question_editor_item_index.set(question_index);
        } else {
            let question = Question {
                question: Default::default(),
                image_url: Default::default(),
                answer_cardinality: Default::default(),
                answers: vec![
                    Answer::default(),
                    Answer::default(),
                    Answer::default(),
                    Answer::default(),
                ],
            };
            question_editor_item.set(question);
            question_editor_item_index.set(None);
        }
        question_editor_visibility.set(true);
    };

    let on_add_question = move |question: Question| {
        let question = Signal::new(question);
        if let Some(index) = question_editor_item_index() {
            questions.write()[index] = question;
        } else {
            questions.write().push(question);
        }
        hide_question_editor.clone()();
    };

    let on_cancel_question = move |()| {
        hide_question_editor.clone()();
    };

    let on_delete_question = move |question_index: usize| {
        questions.write().remove(question_index);
        hide_question_editor.clone()();
    };

    rsx! {
        form {
            class: "flex grow flex-col items-center gap-3",
            id: "category-edit-form",
            autocomplete: "off",
            onsubmit: on_submit_category_form,

            FormTextField {
                name: "title",
                title: "field-title",
                pattern: TITLE_PATTERN,
                required: true,
                initial_value: category_reader.title.as_ref(),
            }
            FormNumField {
                name: "sample_size",
                title: "field-sample-size",
                min: "1",
                max: "1000000",
                step: "1",
                required: true,
                initial_value: Some(category_reader.sample_size)
                    .filter(|sample_size| *sample_size > 0)
                    .map(|sample_size| sample_size.to_string()),
            }
            FormNumField {
                name: "success_score",
                title: "field-success-score",
                min: "1",
                max: "1000000",
                step: "1",
                required: false,
                initial_value: category_reader.success_score.map(|value| format!("{}", value)),
            }
        }
        div { style: "justify-content:center", class: "flex gap-3",
            button {
                onclick: move |event| {
                    event.prevent_default();
                    event.stop_propagation();
                    on_cancel_category.call(());
                },
                name: "cancel-category",
                class: "btn",
                r#type: "button",
                {t!("action-back")}
            }
            button {
                name: "submit-category",
                style: "float: right",
                class: "btn btn-primary",
                form: "category-edit-form",
                r#type: "submit",
                disabled: !writer,
                if category_editor_item_index().is_some() {
                    {t!("action-update-category")}
                } else {
                    {t!("action-add-category")}
                }
            }
        }
        QuestionListView {
            writer,
            questions,
            on_select_question,
            on_delete_question,
        }
        if *question_editor_visibility.read() {
            QuestionEditorView {
                writer,
                question_editor_item,
                question_editor_item_index,
                on_add_question,
                on_cancel_question,
            }
        }
    }
}

#[component]
fn QuestionListView(
    writer: bool,
    questions: Signal<Vec<Signal<Question>>>,
    on_select_question: EventHandler<Option<usize>>,
    on_delete_question: EventHandler<usize>,
) -> Element {
    rsx! {
        div { class: "font-semibold gap-2", {t!("caption-questions")} }
        div {
            section {
                table { class: "entry-table",
                    thead {
                        tr {
                            th { class: "w-12" }
                            th {}
                        }
                    }
                    tbody {
                        for (question_index , question) in questions().iter().copied().enumerate() {
                            QuestionRowView {
                                writer,
                                question_index,
                                question,
                                on_select_question,
                                on_delete_question,
                            }
                        }
                    }
                }
                button {
                    onclick: move |event| {
                        event.prevent_default();
                        event.stop_propagation();
                        on_select_question.clone()(None);
                    },
                    name: "add-question",
                    class: "btn btn-xs btn-ghost",
                    r#type: "button",
                    disabled: !writer,
                    Icon { icon: Icons::Plus, class: "size-4" }
                    {t!("action-add")}
                }
            }
        }
    }
}

#[component]
fn QuestionRowView(
    writer: bool,
    question_index: usize,
    question: Signal<Question>,
    on_select_question: EventHandler<Option<usize>>,
    on_delete_question: EventHandler<usize>,
) -> Element {
    let question_reader = question.read();
    rsx! {
        tr {
            onclick: move |event| {
                event.stop_propagation();
                on_select_question.call(Some(question_index));
            },
            td {
                button {
                    onclick: move |event| {
                        event.prevent_default();
                        event.stop_propagation();
                        on_delete_question.call(question_index);
                    },
                    name: "delete-question",
                    class: "btn btn-xs btn-ghost",
                    r#type: "button",
                    disabled: !writer,
                    Icon { icon: Icons::Close, class: "size-4 text-error" }
                }
            }
            td { {question_reader.question.as_ref()} }
        }
    }
}

#[component]
fn QuestionEditorView(
    writer: bool,
    question_editor_item: Signal<Question>,
    question_editor_item_index: Signal<Option<usize>>,
    on_add_question: EventHandler<Question>,
    on_cancel_question: EventHandler<()>,
) -> Element {
    let mut question = question_editor_item;

    let on_submit_question_form = move |event: Event<FormData>| {
        let mut answer_index = 0;
        let mut answers = vec![];
        loop {
            let answer = event.get_str(format!("answer-{}", answer_index).as_str());
            if let Some(answer) = answer {
                let answer = Answer {
                    answer,
                    correct: event.get_bool(format!("correct-{}", answer_index).as_str()),
                    image_url: None,
                };
                answers.push(answer);
                answer_index += 1;
            } else {
                break;
            }
        }
        let answer_cardinality = if answers.iter().filter(|answer| answer.correct).count() > 1 {
            AnswerCardinality::Multiple
        } else {
            AnswerCardinality::Single
        };
        let question = Question {
            question: event.get_str("question").map(Cow::from).unwrap_or_default(),
            image_url: Default::default(),
            answer_cardinality,
            answers,
        };
        on_add_question.call(question);
    };

    rsx! {
        div { style: "margin-top: 20px", class: "font-semibold", {t!("caption-question-editor")} }
        form {
            id: "question-edit-form",
            autocomplete: "off",
            onsubmit: on_submit_question_form,
            div { class: "col-span-2",
                FormTextField {
                    name: "question",
                    title: "field-question",
                    required: true,
                    initial_value: question.read().question.to_string(),
                }
            }
            table { class: "entry-table",
                thead {
                    tr {
                        th { class: "w-12" }
                        th {}
                        th {}
                    }
                }
                tbody {
                    for (answer_index , answer) in question.read().answers.iter().enumerate() {
                        tr {
                            td {
                                button {
                                    onclick: move |event| {
                                        event.prevent_default();
                                        event.stop_propagation();
                                        question.write().answers.remove(answer_index);
                                    },
                                    name: "delete-answer",
                                    class: "btn btn-xs btn-ghost",
                                    r#type: "button",
                                    disabled: !writer,
                                    Icon {
                                        icon: Icons::Close,
                                        class: "size-4 text-error",
                                    }
                                }
                            }
                            td {
                                FormTextField {
                                    name: format!("answer-{}", answer_index),
                                    title: "field-answer",
                                    required: true,
                                    initial_value: answer.answer.as_ref(),
                                }
                            }
                            td {
                                FormCheckBoxField {
                                    name: format!("correct-{}", answer_index),
                                    title: "field-correct",
                                    initial_checked: answer.correct,
                                }
                            }
                        }
                    }
                }
            }
            button {
                onclick: move |event| {
                    event.prevent_default();
                    event.stop_propagation();
                    question.write().answers.push(Answer::default());
                },
                name: "add-answer",
                class: "btn btn-xs btn-ghost",
                r#type: "button",
                disabled: !writer,
                Icon { icon: Icons::Plus, class: "size-4" }
                {t!("action-add")}
            }
        }
        div { style: "justify-content:center", class: "flex gap-3",
            button {
                onclick: move |event| {
                    event.prevent_default();
                    event.stop_propagation();
                    on_cancel_question.call(());
                },
                name: "cancel-question",
                class: "btn",
                r#type: "button",
                {t!("action-back")}
            }
            button {
                name: "submit-question",
                class: "btn btn-primary",
                form: "question-edit-form",
                r#type: "submit",
                disabled: !writer,
                if question_editor_item_index().is_some() {
                    {t!("action-update-question")}
                } else {
                    {t!("action-add-question")}
                }
            }
        }
    }
}
