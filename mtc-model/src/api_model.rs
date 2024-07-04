use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb_sql::Datetime;
use validator::Validate;

use crate::from_thing;

#[derive(Serialize, Deserialize)]
pub struct ApiModel {
    #[serde(deserialize_with = "from_thing")]
    pub id: String,
    pub slug: String,
    pub fields: Option<Value>,
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Deserialize, Validate)]
pub struct ApiPostModel {
    pub fields: Option<Value>,
}