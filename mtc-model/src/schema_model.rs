use serde::{Deserialize, Serialize};
use surrealdb_sql::Datetime;
use validator::Validate;

use crate::field_model::FieldModel;
use crate::from_thing;

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct SchemaModel {
    #[serde(deserialize_with = "from_thing")]
    pub id: String,
    pub slug: String,
    pub title: String,
    pub is_system: bool,
    pub is_collection: bool,
    pub is_public: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FieldModel>>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
}

impl Default for SchemaModel{
    fn default() -> Self {
        Self {
            id: "".to_string(),
            slug: "".to_string(),
            title: "".to_string(),
            is_system: false,
            is_collection: false,
            is_public: false,
            fields: None,
            created_at: Default::default(),
            updated_at: Default::default(),
            created_by: "".to_string(),
            updated_by: "".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Validate)]
pub struct SchemaCreateModel {
    pub title: String,
    pub is_collection: bool,
    pub is_public: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FieldModel>>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct SchemaUpdateModel {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FieldModel>>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct SchemaFieldsModel {
    pub fields: Option<Vec<FieldModel>>,
}

#[derive(Default, Deserialize, Serialize, Validate)]
pub struct SchemasModel {
    pub schemas: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SchemaListItemModel {
    pub slug: String,
    pub title: String,
}