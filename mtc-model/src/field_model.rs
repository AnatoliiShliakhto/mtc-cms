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
    Bool,
    Int,
    Float,
    DateTime,
    #[default]
    Str,
    Text,
    Html,
    BoolArray,
    IntArray,
    FloatArray,
    StrArray,
    TextArray,
}

impl FromStr for FieldTypeModel {
    type Err = bool;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "bool" => FieldTypeModel::Bool,
            "int" => FieldTypeModel::Int,
            "float" => FieldTypeModel::Float,
            "datetime" => FieldTypeModel::DateTime,
            "text" => FieldTypeModel::Text,
            "html" => FieldTypeModel::Html,
            "bool-array" => FieldTypeModel::BoolArray,
            "int-array" => FieldTypeModel::IntArray,
            "float-array" => FieldTypeModel::FloatArray,
            "str-array" => FieldTypeModel::StrArray,
            "text-array" => FieldTypeModel::TextArray,
            &_ => FieldTypeModel::Str
        })
    }
}

impl Display for FieldTypeModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            FieldTypeModel::Bool => "bool",
            FieldTypeModel::Int => "int",
            FieldTypeModel::Float => "float",
            FieldTypeModel::DateTime => "datetime",
            FieldTypeModel::Text => "text",
            FieldTypeModel::Html => "html",
            FieldTypeModel::BoolArray => "bool-array",
            FieldTypeModel::IntArray => "int-array",
            FieldTypeModel::FloatArray => "float-array",
            FieldTypeModel::StrArray => "str-array",
            FieldTypeModel::TextArray => "text-array",
            _ => "str",
        }.to_string();
        write!(f, "{}", str)
    }
}