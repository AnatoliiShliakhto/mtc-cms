use serde::{Deserialize, Serialize};
use surrealdb_sql::Datetime;

use crate::from_thing;

#[derive(Serialize, Deserialize)]
pub struct PermissionModel {
    #[serde(deserialize_with = "from_thing")]
    pub id: String,
    pub slug: String,
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
}
