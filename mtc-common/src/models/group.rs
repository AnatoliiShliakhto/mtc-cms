use super::*;

/// Group model structure
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Group {
    pub id: Cow<'static, str>,
    pub slug: Cow<'static, str>,
    pub title: Cow<'static, str>,
    pub created_at: Cow<'static, str>,
    pub updated_at: Cow<'static, str>,
    pub created_by: Cow<'static, str>,
    pub updated_by: Cow<'static, str>,
}
