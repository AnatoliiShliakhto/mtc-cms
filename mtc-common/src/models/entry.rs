use super::*;

/// An entry in a collection
#[derive(Default, Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct Entry {
    pub id: Cow<'static, str>,
    pub slug: Cow<'static, str>,
    pub title: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<Value>,
}
