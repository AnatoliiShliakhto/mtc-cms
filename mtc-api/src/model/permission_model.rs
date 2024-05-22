use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;
use validator::Validate;

use crate::model::from_thing;

#[derive(Serialize, Deserialize)]
pub struct PermissionModel {
    #[serde(deserialize_with = "from_thing")]
    pub id: String,
    pub slug: String,
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PermissionsModel {
    pub permissions: Vec<String>,
}