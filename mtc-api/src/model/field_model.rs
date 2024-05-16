use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FieldModel {
    pub order: usize,
    pub name: String,
    pub field_type: FieldTypeModel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FieldTypeModel {
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    Html(String),
}

impl FieldTypeModel {
    fn as_bool(&self) -> bool {
        use FieldTypeModel::*;

        match *self {
            Bool(value) => value,
            _ => false
        }
    }

    fn as_int(&self) -> i64 {
        use FieldTypeModel::*;

        match *self {
            Int(value) => value,
            _ => 0
        }
    }

    fn as_float(&self) -> f64 {
        use FieldTypeModel::*;

        match *self {
            Float(value) => value,
            _ => 0f64
        }
    }

    fn as_string(&self) -> String {
        use FieldTypeModel::*;

        match *self {
            Text(ref value) => value.to_owned(),
            Html(ref value) => value.to_owned(),
            _ => "".to_string()
        }
    }

    fn filed_type(&self) -> String {
        use FieldTypeModel::*;

        match *self {
            Bool(..) => "Bool".to_string(),
            Int(..) => "Int".to_string(),
            Float(..) => "Float".to_string(),
            Text(..) => "Text".to_string(),
            Html(..) => "Html".to_string(),
        }
    }
}