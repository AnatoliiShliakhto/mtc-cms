use super::*;

#[derive(Default, Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct CourseEntry {
    pub id: usize,
    #[serde(borrow)]
    pub title: Cow<'static, str>,
    #[serde(borrow)]
    pub description: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub links: Cow<'static, Option<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub childs: Cow<'static, Option<Vec<usize>>>,
}
