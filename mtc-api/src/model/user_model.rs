use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UserModel {
    #[serde(skip_serializing)]
    pub id: Option<Thing>,
    pub login: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub is_active: bool,
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
}