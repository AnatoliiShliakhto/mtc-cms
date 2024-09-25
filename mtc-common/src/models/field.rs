use super::*;

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Field {
    pub kind: FieldKind,
    pub slug: Cow<'static, str>,
    pub title: Cow<'static, str>,
}
