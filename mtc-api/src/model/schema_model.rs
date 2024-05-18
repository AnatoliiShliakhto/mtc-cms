use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::model::field_model::FieldModel;
use crate::model::from_thing;

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct SchemaModel {
    #[serde(deserialize_with = "from_thing")]
    pub id: String,
    pub name: String,
    pub is_system: bool,
    pub is_collection: bool,
    pub fields: Option<Vec<FieldModel>>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}