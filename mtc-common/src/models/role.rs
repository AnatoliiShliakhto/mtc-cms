use super::*;

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct Role {
    pub id: Cow<'static, str>,
    pub slug: Cow<'static, str>,
    pub title: Cow<'static, str>,
    pub user_access_level: i32,
    pub user_access_all: bool,
    pub permissions: Vec<Cow<'static, str>>,
    pub created_at: Cow<'static, Datetime>,
    pub updated_at: Cow<'static, Datetime>,
    pub created_by: Cow<'static, str>,
    pub updated_by: Cow<'static, str>,
}

impl Default for Role {
    fn default() -> Self {
        Self {
            id: "".into(),
            slug: "".into(),
            title: "".into(),
            user_access_level: 999,
            user_access_all: false,
            permissions: vec![],
            created_at: Default::default(),
            updated_at: Default::default(),
            created_by: "".into(),
            updated_by: "".into(),
        }
    }
}
