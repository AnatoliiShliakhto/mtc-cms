use super::*;
use mtc_common::prelude::UserQuizStatus::Pending;

#[handler(permission = "quizzes::write")]
pub async fn create_quiz_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(mut request): Payload<CreateQuizRequest>,
) {
    let user_login = session.get_auth_login().await?;
    request.created_by = Some(user_login.clone());
    request.updated_by = Some(user_login);
    state.repository.create_quiz(request).await.map(Json)
}

#[handler(permission = "quizzes::write")]
pub async fn update_quiz_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
    Payload(mut request): Payload<UpdateQuizRequest>,
) {
    let user_login = session.get_auth_login().await?;
    request.id = Some(id);
    request.updated_by = Some(user_login);
    state.repository.update_quiz(request).await.map(Json)
}

#[handler(session, permission = "quizzes::delete")]
pub async fn delete_quiz_handler(
    Path(quiz_id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
) {
    state.repository.delete_quiz(quiz_id).await.map(Json)
}

#[handler(session, permission = "quizzes::read")]
pub async fn find_quizzes_handler(state: State<Arc<AppState>>) {
    state.repository.find_quizzes().await.map(Json)
}

#[handler(session, permission = "quizzes::read")]
pub async fn find_quiz_handler(
    Path(quiz_id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
) {
    state.repository.find_quiz(quiz_id).await.map(Json)
}

#[handler(permission = "quizzes::write")]
pub async fn assign_user_quizzes_handler(
    Path(quiz_id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
    Payload(mut request): Payload<AssignUserQuizRequest>,
) {
    let login = session.get_auth_login().await?;
    request.quiz_id = Some(quiz_id);
    request.user_ids = request.user_ids.or_else(|| Some(vec![]));
    request.group_ids = request.group_ids.or_else(|| Some(vec![]));
    request.created_by = Some(login.clone());
    request.updated_by = Some(login);
    state
        .repository
        .assign_user_quizzes(request)
        .await
        .map(Json)
}

#[handler(session, permission = "quizzes::read")]
pub async fn find_user_quizzes_handler(
    Path(user_id): Path<Cow<'static, str>>,
    axum_extra::extract::Query(mut request): axum_extra::extract::Query<FindUserQuizRequest>,
    state: State<Arc<AppState>>,
) {
    request.user_id = Some(user_id);
    request.user_quiz_statuses = request.user_quiz_statuses.or_else(|| Some(vec![Pending]));
    state.repository.find_user_quizzes(request).await.map(Json)
}

#[handler]
pub async fn find_my_quizzes_handler(
    axum_extra::extract::Query(mut request): axum_extra::extract::Query<FindUserQuizRequest>,
    state: State<Arc<AppState>>,
    session: Session,
) {
    let user_id = session.get_auth_id().await?;
    request.user_id = Some(user_id);
    request.user_quiz_statuses = request.user_quiz_statuses.or_else(|| Some(vec![Pending]));
    state.repository.find_user_quizzes(request).await.map(Json)
}

#[handler]
pub async fn complete_my_quiz_handler(
    Path(quiz_id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
    Payload(quiz_result): Payload<UserQuizResult>,
) {
    let user_id = session.get_auth_id().await?;
    let user_login = session.get_auth_login().await?;
    let request = CompleteUserQuizRequest {
        quiz_id: Some(quiz_id),
        user_id: Some(user_id),
        updated_by: Some(user_login),
        quiz_result,
    };
    state.repository.complete_user_quiz(request).await.map(Json)
}

#[handler(permission = "quizzes::write")]
pub async fn complete_user_quiz_handler(
    Path((quiz_id, user_id)): Path<(Cow<'static, str>, Cow<'static, str>)>,
    state: State<Arc<AppState>>,
    session: Session,
    Payload(quiz_result): Payload<UserQuizResult>,
) {
    let user_login = session.get_auth_login().await?;
    let request = CompleteUserQuizRequest {
        quiz_id: Some(quiz_id),
        user_id: Some(user_id),
        updated_by: Some(user_login),
        quiz_result,
    };
    state.repository.complete_user_quiz(request).await.map(Json)
}
