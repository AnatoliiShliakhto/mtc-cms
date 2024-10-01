use super::*;

#[derive(Default, Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct SchemaEntryDetails {
    pub kind: SchemaKind,
    pub permission: Cow<'static, str>,
}
