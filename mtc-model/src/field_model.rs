use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FieldModel {
    pub slug: String,
    pub title: String,
    #[serde(rename = "type")]
    pub field_type: FieldTypeModel,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FieldTypeModel {
    #[default]
    Str,
    Text,
    Html,
    Decimal,
    DateTime,
}

impl FromStr for FieldTypeModel {
    type Err = bool;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "text" => FieldTypeModel::Text,
            "html" => FieldTypeModel::Html,
            "decimal" => FieldTypeModel::Decimal,
            "datetime" => FieldTypeModel::DateTime,
            &_ => FieldTypeModel::Str
        })
    }
}

impl Display for FieldTypeModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            FieldTypeModel::Text => "text",
            FieldTypeModel::Html => "html",
            FieldTypeModel::DateTime => "datetime",
            FieldTypeModel::Decimal => "decimal",
            _ => "str",
        }.to_string();
        write!(f, "{}", str)
    }
}
