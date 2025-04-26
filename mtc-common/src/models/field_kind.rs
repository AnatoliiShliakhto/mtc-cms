use super::*;

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FieldKind {
    #[default]
    Str,
    Text,
    Html,
    PlainHtml,
    Links,
    Course,
    Decimal,
    DateTime,
}

impl FromStr for FieldKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "text" => FieldKind::Text,
            "html" => FieldKind::Html,
            "plainhtml" => FieldKind::PlainHtml,
            "links" => FieldKind::Links,
            "course" => FieldKind::Course,
            "decimal" => FieldKind::Decimal,
            "datetime" => FieldKind::DateTime,
            &_ => FieldKind::Str
        })
    }
}

impl Display for FieldKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            FieldKind::Text => "text",
            FieldKind::Html => "html",
            FieldKind::PlainHtml => "plainhtml",
            FieldKind::Links => "links",
            FieldKind::Course => "course",
            FieldKind::DateTime => "datetime",
            FieldKind::Decimal => "decimal",
            _ => "str",
        }.to_string();
        write!(f, "{}", str)
    }
}
