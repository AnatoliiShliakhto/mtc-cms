use super::*;

#[derive(Default, Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct SchemaModel {
    pub id: Cow<'static, str>,
    pub kind: SchemaKind,
    pub slug: Cow<'static, str>,
    pub title: Cow<'static, str>,
    pub permission: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Field>>,
    pub created_at: Cow<'static, Datetime>,
    pub updated_at: Cow<'static, Datetime>,
    pub created_by: Cow<'static, str>,
    pub updated_by: Cow<'static, str>,
}