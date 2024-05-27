use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::sql::Datetime;
use validator::Validate;

use crate::model::from_thing;

#[derive(Serialize, Deserialize)]
pub struct ApiModel {
    #[serde(deserialize_with = "from_thing")]
    pub id: String,
    pub slug: String,
    pub fields: Option<Value>,
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
}

#[derive(Deserialize, Validate)]
pub struct ApiPostModel {
    pub fields: Option<Value>,
}