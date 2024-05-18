use serde::{Deserialize, Deserializer, Serialize};
use surrealdb::sql::Thing;

pub mod role_model;
pub mod permission_model;
pub mod response_model;
pub mod request_model;
pub mod user_model;
pub mod auth_model;
pub mod group_model;
pub mod field_model;
pub mod single_type_model;
pub mod schema_model;
pub mod pagination_model;

#[derive(Deserialize, Serialize)]
pub struct StringListModel {
    pub items: Vec<String>,
}

pub fn from_thing<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
{
    let thing: Thing = Deserialize::deserialize(deserializer)?;
    Ok(thing.id.to_string())
}
