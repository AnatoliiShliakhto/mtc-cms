use super::*;

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: Cow<'static, str>,
    pub login: Cow<'static, str>,
    #[serde(skip_serializing, default)]
    pub password: Cow<'static, str>,
    pub group: Cow<'static, str>,
    pub blocked: bool,
    pub access_level: i32,
    pub access_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access: Cow<'static, Option<Datetime>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Cow<'static, Option<Value>>,
    pub created_at: Cow<'static, Datetime>,
    pub updated_at: Cow<'static, Datetime>,
    pub created_by: Cow<'static, str>,
    pub updated_by: Cow<'static, str>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: "".into(),
            login: "".into(),
            password: "".into(),
            group: "".into(),
            blocked: false,
            access_level: 999,
            access_count: 0,
            last_access: Cow::Borrowed(&None),
            fields: Cow::Borrowed(&None),
            created_at: Default::default(),
            updated_at: Default::default(),
            created_by: "".into(),
            updated_by: "".into(),
        }
    }
}
