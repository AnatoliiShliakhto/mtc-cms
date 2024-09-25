use super::*;

#[derive(Default, Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum SchemaKind {
    System = 0,
    User = 1,
    #[default]
    Page = 2,
    Pages = 3,
    Links = 4,
    Course = 5,
    Quiz = 6,
}

impl FromStr for SchemaKind {
    type Err = bool;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "user" => SchemaKind::User,
            "page" => SchemaKind::Page,
            "pages" => SchemaKind::Pages,
            "links" => SchemaKind::Links,
            "course" => SchemaKind::Course,
            "quiz" => SchemaKind::Quiz,
            &_ => SchemaKind::System,
        })
    }
}

impl Display for SchemaKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            SchemaKind::System => "system",
            SchemaKind::User => "user",
            SchemaKind::Page => "page",
            SchemaKind::Pages => "pages",
            SchemaKind::Links => "links",
            SchemaKind::Course => "course",
            SchemaKind::Quiz => "quiz"
        }.to_string();
        write!(f, "{}", str)
    }
}

impl From<SchemaKind> for &str {
    fn from(value: SchemaKind) -> Self {
        match value {
            SchemaKind::System => "system",
            SchemaKind::User => "user",
            SchemaKind::Page => "page",
            SchemaKind::Pages => "pages",
            SchemaKind::Links => "links",
            SchemaKind::Course => "course",
            SchemaKind::Quiz => "quiz"
        }
    }
}