use super::*;

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct CourseEntry {
    pub id: usize,
    pub parent: usize,
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Value>,
}

impl Default for CourseEntry {
    fn default() -> Self {
        Self {
            id: 0,
            parent: u32::MAX as usize,
            title: Default::default(),
            description: Default::default(),
            links: None,
        }
    }
}