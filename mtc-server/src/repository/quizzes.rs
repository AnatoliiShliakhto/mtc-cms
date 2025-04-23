use super::*;
use crate::prelude::{info, Repository};
use ::mtc_common::prelude::{Entry, Quiz};

pub trait QuizRepository {
    async fn create_quiz(&self, request: CreateQuizRequest) -> Result<Quiz>;
    async fn update_quiz(&self, request: UpdateQuizRequest) -> Result<Quiz>;
    async fn delete_quiz(&self, quiz_id: impl ToString) -> Result<Quiz>;
    async fn find_quizzes(&self) -> Result<Vec<Entry>>;
    async fn find_quiz(&self, quiz_id: impl ToString) -> Result<Quiz>;
    async fn assign_user_quizzes(&self, request: AssignUserQuizRequest) -> Result<Vec<UserQuiz>>;
    async fn find_user_quizzes(&self, request: FindUserQuizRequest) -> Result<Vec<UserQuiz>>;
    async fn complete_user_quiz(&self, request: CompleteUserQuizRequest) -> Result<UserQuiz>;
}

impl QuizRepository for Repository {
    async fn create_quiz(&self, mut request: CreateQuizRequest) -> Result<Quiz> {
        let query = r#"
        BEGIN TRANSACTION;
            $category_record_ids = $categories.map(|$category| (CREATE categories CONTENT $category).id[0]);
            LET $quiz_record = CREATE quizzes CONTENT $quiz;
            UPDATE $quiz_record SET category_ids = $category_record_ids;
            RETURN SELECT *, id.id() as id, category_ids.*.{id: id.id(), title, success_score, sample_size, questions} as categories FROM $quiz_record.id;
        COMMIT TRANSACTION;
        "#;
        self.database
            .query(query)
            .bind(("categories", request.categories.take()))
            .bind(("quiz", request))
            .await
            .map(take_successful_response)??
            .take::<Option<Quiz>>(0)
            .map(|quiz_opt| {
                if let Some(quiz) = quiz_opt {
                    info!("Quiz created: id={}, slug={}", &quiz.id, &quiz.slug);
                    Ok(quiz)
                } else {
                    Err(DatabaseError::SomethingWentWrong.into())
                }
            })?
    }

    async fn update_quiz(&self, mut request: UpdateQuizRequest) -> Result<Quiz> {
        let query = r#"
        BEGIN TRANSACTION;
            LET $quiz_record = SELECT * FROM ONLY type::thing('quizzes', $quiz_patch.id);
            IF $quiz_record = NONE THEN {
                RETURN NONE
            } END;
            IF count($categories) > 0 THEN {
                DELETE $quiz_record.category_ids;
                $category_record_ids = $categories.map(|$category| (CREATE categories CONTENT $category).id[0]);
                UPDATE $quiz_record SET category_ids = $category_record_ids;
            } END;
            UPDATE $quiz_record MERGE $quiz_patch;
            RETURN SELECT *, id.id() as id, category_ids.*.{id: id.id(), title, success_score, sample_size, questions} as categories FROM $quiz_record.id;
        COMMIT TRANSACTION;
        "#;
        self.database
            .query(query)
            .bind(("categories", request.categories.take()))
            .bind(("quiz_patch", request))
            .await
            .map(take_successful_response)??
            .take::<Option<Quiz>>(0)
            .map(|quiz_opt| {
                if let Some(quiz) = quiz_opt {
                    info!("Quiz updated: id={}, slug={}", &quiz.id, &quiz.slug);
                    Ok(quiz)
                } else {
                    Err(DatabaseError::EntryNotFound.into())
                }
            })?
    }

    async fn delete_quiz(&self, quiz_id: impl ToString) -> Result<Quiz> {
        let query = r#"
        BEGIN TRANSACTION;
            LET $quiz_record = SELECT *, [] AS categories FROM ONLY type::thing('quizzes', $quiz_id);
            IF $quiz_record = NONE THEN {
                RETURN NONE
            } END;
            DELETE $quiz_record.category_ids;
            DELETE $quiz_record;
            RETURN SELECT *, id.id() as id FROM $quiz_record;
        COMMIT TRANSACTION;
        "#;
        self.database
            .query(query)
            .bind(("quiz_id", quiz_id.to_string()))
            .await
            .map(take_successful_response)??
            .take::<Option<Quiz>>(0)
            .map(|quiz_opt| {
                if let Some(quiz) = quiz_opt {
                    info!("Quiz deleted: id={}, slug={}", quiz.id, quiz.slug);
                    Ok(quiz)
                } else {
                    Err(DatabaseError::EntryNotFound.into())
                }
            })?
    }

    async fn find_quizzes(&self) -> Result<Vec<Entry>> {
        let query = r#"
        SELECT *, id.id() as id FROM quizzes;
        "#;
        self.database
            .query(query)
            .await
            .map(take_successful_response)??
            .take::<Vec<Entry>>(0)
            .map(|entries| {
                info!("Quizzes found: number_of_records={}", entries.len());
                Ok(entries)
            })?
    }

    async fn find_quiz(&self, quiz_id: impl ToString) -> Result<Quiz> {
        let query = r#"
        SELECT *, id.id() as id, category_ids.*.{id: id.id(), title, success_score, sample_size, questions} as categories FROM ONLY type::thing('quizzes', $quiz_id);
        "#;
        self.database
            .query(query)
            .bind(("quiz_id", quiz_id.to_string()))
            .await
            .map(take_successful_response)??
            .take::<Option<Quiz>>(0)
            .map(|quiz_opt| match quiz_opt {
                Some(quiz) => {
                    info!("Quiz found: id={}, slug={}", &quiz.id, &quiz.slug);
                    Ok(quiz)
                }
                None => Err(DatabaseError::EntryNotFound.into()),
            })?
    }

    async fn assign_user_quizzes(&self, request: AssignUserQuizRequest) -> Result<Vec<UserQuiz>> {
        let quiz = self.find_quiz(request.quiz_id.clone().unwrap()).await?;
        let query = r#"
        BEGIN TRANSACTION;
            LET $quiz_record_id = type::thing('quizzes', $request.quiz_id);
            LET $user_record_ids = $request.user_ids.map(|$user_id| (type::thing('users', $user_id)));
            LET $group_record_ids = $request.group_ids.map(|$group_id| (type::thing('groups', $group_id)));
            LET $group_user_record_ids = array::flatten(
                (SELECT <-user_groups<-users[WHERE blocked = false].* AS user_record_ids FROM $group_record_ids).user_record_ids
             );
            LET $existent_quiz_user_record_ids = (SELECT in as user_record_id FROM user_quizzes WHERE out.id = $quiz_record_id).user_record_id;
            LET $new_quiz_user_record_ids = array::complement(
                array::add($user_record_ids ?? [], $group_user_record_ids ?? []), $existent_quiz_user_record_ids ?? []
            );
            FOR $user_record_id IN $new_quiz_user_record_ids {
                RELATE $user_record_id -> user_quizzes:ulid() -> $quiz_record_id SET
                    created_by = $request.created_by,
                    updated_by = $request.updated_by,
                    quiz_settings.scoring_system = $scoring_system,
                    quiz_settings.success_scores = $success_scores,
                    quiz_settings.active_from = type::datetime($request.active_from),
                    quiz_settings.expired_from = type::datetime($request.expired_from),
                    quiz_result.status = 'Pending'
            };
            RETURN SELECT *, id.id() AS id, in.id.id() AS user_id, out.id.id() AS quiz_id
            FROM user_quizzes WHERE in.id IN $new_quiz_user_record_ids AND out.id = $quiz_record_id;
        COMMIT TRANSACTION;
        "#;
        self.database
            .query(query)
            .bind(("request", request))
            .bind(("success_scores", quiz.success_scores()))
            .bind(("scoring_system", quiz.scoring_system))
            .await
            .map(take_successful_response)??
            .take::<Vec<UserQuiz>>(0)
            .map(|user_quizzes| {
                info!(
                    "Quiz assigned: quiz_id={}, number_of_users={}",
                    quiz.id,
                    user_quizzes.len()
                );
                Ok(user_quizzes)
            })?
    }

    async fn find_user_quizzes(&self, request: FindUserQuizRequest) -> Result<Vec<UserQuiz>> {
        let query = r#"
        BEGIN TRANSACTION;
            LET $now = time::now();
            LET $user_record_id = type::thing('users', $request.user_id);
            RETURN SELECT *, id.id() AS id, in.id.id() AS user_id, out.id.id() AS quiz_id FROM user_quizzes
            WHERE in.id = $user_record_id AND $now < quiz_settings.expired_from AND quiz_result.status IN $request.user_quiz_statuses;
        COMMIT TRANSACTION;
        "#;
        self.database
            .query(query)
            .bind(("request", request))
            .await
            .map(take_successful_response)??
            .take::<Vec<UserQuiz>>(0)
            .map(|user_quizzes| {
                info!(
                    "User quizzes found: number_of_quizzes={}",
                    user_quizzes.len()
                );
                Ok(user_quizzes)
            })?
    }

    async fn complete_user_quiz(&self, request: CompleteUserQuizRequest) -> Result<UserQuiz> {
        let query = r#"
        BEGIN TRANSACTION;
            LET $quiz_record_id = type::thing('quizzes', $request.quiz_id);
            LET $user_record_id = type::thing('users', $request.user_id);
            LET $user_quiz_record = (UPDATE user_quizzes SET
                quiz_result.status = $request.quiz_result.status,
                quiz_result.actual_scores = $request.quiz_result.actual_scores,
                quiz_result.started_at = type::datetime($request.quiz_result.started_at),
                quiz_result.finished_at = type::datetime($request.quiz_result.finished_at),
                updated_by = $request.updated_by
            WHERE in.id = $user_record_id AND out.id = $quiz_record_id)[0];
            IF $user_quiz_record = NONE THEN {
                RETURN NONE
            } END;
            RETURN SELECT *, id.id() as id, in.id.id() AS user_id, out.id.id() AS quiz_id FROM ONLY $user_quiz_record;
        COMMIT TRANSACTION;
        "#;
        let quiz_id = request.quiz_id.clone().unwrap();
        let user_id = request.user_id.clone().unwrap();
        self.database
            .query(query)
            .bind(("request", request))
            .await
            .map(take_successful_response)??
            .take::<Option<UserQuiz>>(0)
            .map(|user_quiz_opt| {
                if let Some(user_quiz) = user_quiz_opt {
                    info!("Quiz completed: quiz_id={}, user_id={}", quiz_id, user_id);
                    Ok(user_quiz)
                } else {
                    Err(DatabaseError::EntryNotFound.into())
                }
            })?
    }
}

fn take_successful_response(mut response: surrealdb::Response) -> Result<surrealdb::Response> {
    let errors = response.take_errors();
    if errors.is_empty() {
        Ok(response)
    } else {
        let errors = errors
            .values()
            .map(|error| error.to_string())
            .collect::<String>();
        error!("Error occurred: {}", errors);
        Err(DatabaseError::SomethingWentWrong.into())
    }
}
