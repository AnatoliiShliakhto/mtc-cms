use super::*;

#[derive(Default, Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct CourseEntry {
    pub id: usize,
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Cow<'static, Option<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub childs: Cow<'static, Option<Vec<usize>>>,
}
