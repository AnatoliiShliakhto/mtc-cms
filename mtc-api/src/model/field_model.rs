use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FieldModel {
    pub name: String,
    pub order: usize,
    pub field_type: FieldTypeModel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FieldTypeModel {
    Bool,
    Int,
    Float,
    DateTime,
    Str,
    Text,
    Html,
    BoolArray,
    IntArray,
    FloatArray,
    StrArray,
    TextArray,
}