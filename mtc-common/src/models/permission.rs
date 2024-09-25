use super::*;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Permission {
    pub id: Cow<'static, str>,
    pub slug: Cow<'static, str>,
    pub created_by: Cow<'static, str>,
    pub created_at: Cow<'static, Datetime>,
}