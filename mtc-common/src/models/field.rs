use super::*;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Field {
    pub kind: FieldKind,
    pub slug: Cow<'static, str>,
    pub title: Cow<'static, str>,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            kind: FieldKind::default(),
            slug: Cow::Borrowed(""),
            title: Cow::Borrowed(""),
        }
    }
}