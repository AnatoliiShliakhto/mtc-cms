use super::*;

/// Search index structure
#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct SearchIdx {
    pub kind: SearchKind,
    #[serde(borrow)]
    pub title: Cow<'static, str>,
    #[serde(borrow)]
    pub url: Cow<'static, str>,
    #[serde(borrow)]
    pub permission: Cow<'static, str>,
}