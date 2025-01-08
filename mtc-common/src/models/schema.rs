use super::*;

/// Schema model structure
#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct Schema {
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

impl Default for Schema {
    fn default() -> Self {
        Self {
            id: Default::default(),
            kind: SchemaKind::Page,
            slug: Default::default(),
            title: Default::default(),
            permission: Cow::Borrowed(PERMISSION_PUBLIC),
            fields: None,
            created_at: Default::default(),
            updated_at: Default::default(),
            created_by: Default::default(),
            updated_by: Default::default(),
        }
    }
}