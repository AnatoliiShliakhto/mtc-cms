use super::*;

/// Link entry
#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LinkEntry {
    pub title: Cow<'static, str>,
    pub url: Cow<'static, str>,
}