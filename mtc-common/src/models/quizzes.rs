use super::*;

/// Quiz create request model structure
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateQuizRequest {
    pub slug: Cow<'static, str>,
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub duration_ms: u32,
    pub scoring_system: ScoringSystem,
    pub success_score: u16,
    pub categories: Option<Vec<Category>>,
    pub created_by: Option<Cow<'static, str>>,
    pub updated_by: Option<Cow<'static, str>>,
}

/// Quiz patch request model structure
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateQuizRequest {
    pub id: Option<Cow<'static, str>>,
    pub slug: Option<Cow<'static, str>>,
    pub title: Option<Cow<'static, str>>,
    pub description: Option<Cow<'static, str>>,
    pub duration_ms: Option<u32>,
    pub scoring_system: Option<ScoringSystem>,
    pub success_score: Option<u16>,
    pub categories: Option<Vec<Category>>,
    pub updated_by: Option<Cow<'static, str>>,
}

/// Quiz model structure
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Quiz {
    pub id: Cow<'static, str>,
    pub slug: Cow<'static, str>,
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub duration_ms: u32,
    pub scoring_system: ScoringSystem,
    pub success_score: u16,
    pub categories: Vec<Category>,
    pub created_by: Cow<'static, str>,
    pub created_at: Cow<'static, str>,
    pub updated_by: Cow<'static, str>,
    pub updated_at: Cow<'static, str>,
}

impl Quiz {
    pub fn success_scores(&self) -> Vec<u16> {
        match self.scoring_system {
            ScoringSystem::CategorySuccessScore | ScoringSystem::CategorySuccessScoreRate => self
                .categories
                .iter()
                .map(|category: &Category| category.success_score.unwrap_or(self.success_score))
                .collect(),
            _ => vec![self.success_score],
        }
    }
}

/// Quiz scoring system enum
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ScoringSystem {
    #[default]
    /// Score is set for quiz, quiz passed if score reached
    TotalSuccessScore,
    /// Score rate is set for quiz, quiz passed if score reached
    TotalSuccessScoreRate,
    /// Score is set per category, quiz passed if all category scores reached
    CategorySuccessScore,
    /// Score rate is set per category, quiz passed if all category score rate reached
    CategorySuccessScoreRate,
}

/// Quiz category model structure
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Category {
    pub id: Option<Cow<'static, str>>,
    pub title: Cow<'static, str>,
    pub success_score: Option<u16>,
    pub sample_size: u16,
    pub questions: Vec<Question>,
}

/// Category question model structure
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Question {
    pub question: Cow<'static, str>,
    pub image_url: Option<Cow<'static, str>>,
    pub answer_cardinality: AnswerCardinality,
    pub answers: Vec<Answer>,
}

/// Question answer cardinality enum
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AnswerCardinality {
    #[default]
    /// Question can have only one correct answer
    Single,
    /// Question can have multiple correct answers
    Multiple,
}

/// Question answer model structure
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Answer {
    pub answer: Cow<'static, str>,
    pub image_url: Option<Cow<'static, str>>,
    pub correct: bool,
}

/// Assign user quiz request model structure
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AssignUserQuizRequest {
    pub quiz_id: Option<Cow<'static, str>>,
    pub user_ids: Option<Vec<Cow<'static, str>>>,
    pub group_ids: Option<Vec<Cow<'static, str>>>,
    pub active_from: Cow<'static, str>,
    pub expired_from: Cow<'static, str>,
    pub created_by: Option<Cow<'static, str>>,
    pub updated_by: Option<Cow<'static, str>>,
}

/// User quiz model structure
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserQuiz {
    pub id: Cow<'static, str>,
    pub quiz_id: Cow<'static, str>,
    pub user_id: Cow<'static, str>,
    pub quiz_settings: QuizSettings,
    pub quiz_result: UserQuizResult,
    pub created_by: Cow<'static, str>,
    pub created_at: Cow<'static, str>,
    pub updated_by: Cow<'static, str>,
    pub updated_at: Cow<'static, str>,
}

/// User quiz settings model structure
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct QuizSettings {
    pub scoring_system: ScoringSystem,
    pub success_scores: Vec<u16>,
    pub active_from: Cow<'static, str>,
    pub expired_from: Cow<'static, str>,
}

/// Complete user quiz request model structure
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CompleteUserQuizRequest {
    pub quiz_id: Option<Cow<'static, str>>,
    pub user_id: Option<Cow<'static, str>>,
    pub quiz_result: UserQuizResult,
    pub updated_by: Option<Cow<'static, str>>,
}

/// Quiz result request model structure
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserQuizResult {
    pub status: UserQuizStatus,
    pub actual_scores: Option<Vec<u16>>,
    pub started_at: Option<Cow<'static, str>>,
    pub finished_at: Option<Cow<'static, str>>,
}

/// Find user quiz request model structure
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FindUserQuizRequest {
    pub user_id: Option<Cow<'static, str>>,
    pub user_quiz_statuses: Option<Vec<UserQuizStatus>>,
}

/// User quiz status enum
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserQuizStatus {
    #[default]
    Pending,
    Failed,
    Passed,
}
