use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::model::from_thing;

#[derive(Serialize, Deserialize)]
pub struct PermissionModel {
    #[serde(deserialize_with = "from_thing")]
    pub id: String,
    pub name: String,
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
}

#[derive(Deserialize)]
pub struct PermissionsModel {
    pub permissions: Vec<String>,
}

#[derive(Deserialize)]
pub struct PermissionCreateModel {
    pub name: String,
}