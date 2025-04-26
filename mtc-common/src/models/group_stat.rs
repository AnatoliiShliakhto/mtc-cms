use super::*;

/// Group model structure
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupStat {
    pub title: Cow<'static, str>,
    pub online: i64,
    pub total: i64,
}
