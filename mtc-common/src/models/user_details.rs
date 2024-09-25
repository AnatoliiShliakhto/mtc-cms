use super::*;

#[derive(Serialize, Default, Debug, Deserialize, Clone, PartialEq)]
pub struct UserDetails {
    #[serde(borrow)]
    pub rank: Cow<'static, str>,
    #[serde(borrow)]
    pub name: Cow<'static, str>,
}
