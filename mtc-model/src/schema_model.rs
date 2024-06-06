use serde::{Deserialize, Serialize};
use surrealdb_sql::Datetime;
use validator::Validate;

use crate::field_model::FieldModel;
use crate::from_thing;

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct SchemaModel {
    #[serde(deserialize_with = "from_thing")]
    pub id: String,
    pub slug: String,
    pub title: String,
    pub is_system: bool,
    pub is_collection: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FieldModel>>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Deserialize, Validate)]
pub struct SchemaCreateModel {
    pub title: String,
    pub is_collection: bool,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct SchemaUpdateModel {
    pub title: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct SchemaFieldsModel {
    pub fields: Option<Vec<FieldModel>>,
}