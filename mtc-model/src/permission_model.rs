use serde::{Deserialize, Serialize};
use surrealdb_sql::Datetime;
use validator::Validate;
use crate::from_thing;

#[derive(Default, Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct PermissionModel {
    #[serde(deserialize_with = "from_thing")]
    pub id: String,
    pub slug: String,
    pub created_by: String,
    pub created_at: Datetime,
}

#[derive(Deserialize, Serialize, Validate, Clone)]
pub struct PermissionDtoModel {
    pub slug: String,
}