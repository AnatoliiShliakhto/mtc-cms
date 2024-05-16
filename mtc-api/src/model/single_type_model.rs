use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;
use validator::Validate;

use crate::model::field_model::FieldModel;
use crate::model::from_thing;

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct SingleTypeModel {
    #[serde(deserialize_with = "from_thing")]
    pub id: String,
    pub api: String,
    pub fields: Option<Vec<FieldModel>>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Deserialize, Validate)]
pub struct SingleTypeCreateModel {
    pub api: String,
}