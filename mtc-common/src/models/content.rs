use super::*;

#[derive(Default, Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct Content {
    pub id: Cow<'static, str>,
    pub slug: Cow<'static, str>,
    pub title: Cow<'static, str>,
    pub data: Cow<'static, Option<Value>>,
    pub published: bool,
    pub created_at: Cow<'static, Datetime>,
    pub updated_at: Cow<'static, Datetime>,
    pub created_by: Cow<'static, str>,
    pub updated_by: Cow<'static, str>,
}
