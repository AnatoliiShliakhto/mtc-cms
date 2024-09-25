use super::*;

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LinkModel {
    #[serde(borrow)]
    pub title: Cow<'static, str>,
    #[serde(borrow)]
    pub url: Cow<'static, str>,
}