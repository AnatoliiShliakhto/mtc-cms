use super::*;

#[derive(Serialize, Default, Debug, Deserialize, Clone, PartialEq)]
pub struct UserState {
    #[serde(borrow)]
    pub login: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub group: Cow<'static, Option<Cow<'static, str>>>,
    pub blocked: bool,
    pub access_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub last_access: Cow<'static, Option<Datetime>>,
}
