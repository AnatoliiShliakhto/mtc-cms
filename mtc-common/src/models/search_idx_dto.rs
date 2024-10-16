use super::*;

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, Ord, Eq, PartialOrd)]
pub struct SearchIdxDto {
    pub kind: SearchKind,
    pub title: Cow<'static, str>,
    pub url: Cow<'static, str>,
}